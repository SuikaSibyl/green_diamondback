(* a diamondback reference interpreter *)
(* Mark Barbone 5/4/23 *)

module SExpr = struct
  type s =
    | List of s list
    | Symbol of string
    | Num of int

  exception Syntax

  let parse s =
    let s = s ^ ")\x00" in
    let isdigit c = '0' <= c && c <= '9' in
    let isident c = 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z'
      || isdigit c || String.contains "+-*><=!" c in
    let rec ws i =
      if s.[i] = ' ' || s.[i] = '\n' || s.[i] = '\t' then ws (i+1) else i in
    let rec end_of_ident i = if isident s.[i] then end_of_ident (i+1) else i in
    let rec end_of_num i =
      if isdigit s.[i] then end_of_num (i+1)
      else if isident s.[i] then true, end_of_ident (i+1)
      else false, i in
    let rec item i =
      if s.[i] = '(' then
        let xs, i = rest_of_list @@ ws (i+1) in List(xs), i
      else if isdigit s.[i] || s.[i] = '-' then
        let ident, i' = end_of_num (i+1) in
        if ident || i' = i+1 && s.[i] = '-' then
          Symbol (String.sub s i (i' - i)), ws i'
        else match int_of_string_opt @@ String.sub s i (i' - i) with
        | Some x -> Num x, ws i'
        | None -> raise Syntax
      else if isident s.[i] then
        let i' = end_of_ident (i+1) in Symbol (String.sub s i (i' - i)), ws i'
      else
        raise Syntax
    and rest_of_list i =
      if s.[i] = ')' then
        [], ws (i+1)
      else
        let x, i = item i in
        let xs, i = rest_of_list i in
        (x :: xs), i in
    let x, k = rest_of_list 0 in
    if k <> String.length s - 1 then
      raise Syntax
    else x
end

module AST = struct
  open SExpr

  type t =
    | Num of int
    | True | False
    | Input
    | Var of string
    | Let of (string * t) list * t
    | Op1 of op1 * t
    | Op2 of op2 * t * t
    | Set of string * t
    | If of t * t * t
    | Block of t list * t
    | Loop of t
    | Call of string * t list
  and op1 = Add1 | Sub1 | Isnum | Isbool | Print | Break
  and op2 = Add | Sub | Mul | Lt | Gt | Lte | Gte | Eq

  type defn = { name: string; args: string list; body: t }

  let kws =
    [ "true"; "false"; "input"
    ; "let"; "set!"; "if"; "block"; "loop"; "break"; "fun"
    ; "add1"; "sub1"; "isnum"; "isbool"; "print"
    ; "+"; "-"; "*"; "<"; ">"; "<="; ">="; "="]

  let sexp_to_ast s =
    let rec init_last = function
      | [] -> raise Syntax
      | [x] -> [], x
      | x::xs -> let a, b = init_last xs in (x::a), b in
    let op1 = function
      | "add1" -> Add1 | "sub1" -> Sub1 | "isnum" -> Isnum | "isbool" -> Isbool
      | "print" -> Print | "break" -> Break | _ -> raise Syntax in
    let op2 = function
      | "+" -> Add | "-" -> Sub | "*" -> Mul | "<" -> Lt | ">" -> Gt
      | "<=" -> Lte | ">=" -> Gte | "=" -> Eq | _ -> raise Syntax in
    let rec exp = function
      | Symbol "true" -> True | Symbol "false" -> False
      | Symbol "input" -> Input
      | Symbol x -> if List.mem x kws then raise Syntax else Var x
      | Num x -> Num x
      | List[Symbol "let"; List binds; body] ->
          let bind = function
            | List [Symbol s; x] when not (List.mem s kws)-> s, exp x
            | _ -> raise Syntax in
          if binds = [] then raise Syntax; Let(List.map bind binds, exp body)
      | List(Symbol "let" :: _) -> raise Syntax
      | List[Symbol "set!"; Symbol var; body] ->
          if List.mem var kws then raise Syntax else Set(var, exp body)
      | List(Symbol "set!" :: _) -> raise Syntax
      | List[Symbol "if"; a; b; c] -> If(exp a, exp b, exp c)
      | List(Symbol "if" :: _) -> raise Syntax
      | List(Symbol "block" :: contents) ->
          let ss, e = init_last @@ List.map exp contents in
          Block(ss, e)
      | List[Symbol "loop"; body] -> Loop(exp body)
      | List(Symbol "loop" :: _) -> raise Syntax
      | List[Symbol op; body] when List.mem op kws -> Op1(op1 op, exp body)
      | List[Symbol op; x; y] when List.mem op kws -> Op2(op2 op, exp x, exp y)
      | List(Symbol fn :: args) ->
          if List.mem fn kws then raise Syntax else Call(fn, List.map exp args)
      | List _ -> raise Syntax in
    let fn = function
      | List [Symbol "fun"; List(Symbol name :: args); body] ->
          let arg = function
            | Symbol s when not (List.mem s kws) -> s
            | _ -> raise Syntax in
          { name; args = List.map arg args; body = exp body }
      | _ -> raise Syntax in
    let fns, body = init_last s in
    List.map fn fns, exp body


  exception BadProgram

  let check_program fns main =
    let no_dups xs =
      if List.(length (sort_uniq Stdlib.compare xs) <> length xs) then
        raise BadProgram in
    let fn_arities =
      List.map (fun { name; args; _ } -> name, List.length args) fns in
    no_dups (List.map fst fn_arities);
    let rec exp ctx = function
      | Num _ | True | False -> ()
      | Var v -> if not (List.mem v ctx) then raise BadProgram
      | Input -> if not (List.mem "[input]" ctx) then raise BadProgram
      | Let(binds, body) ->
          no_dups (List.map fst binds);
          let ctx =
            List.fold_left (fun ctx (a,v) -> exp ctx v; a::ctx) ctx binds in
          exp ctx body
      | Op1(Break, _) when not (List.mem "[loop]" ctx) -> raise BadProgram
      | Op1(_, x) -> exp ctx x
      | Op2(_, x, y) -> exp ctx x; exp ctx y
      | Set(name, x) ->
          if not (List.mem name ctx) then raise BadProgram; exp ctx x
      | If(cond, a, b) -> exp ctx cond; exp ctx a; exp ctx b
      | Block(ss, e) -> List.iter (exp ctx) ss; exp ctx e
      | Loop l -> exp ("[loop]" :: ctx) l
      | Call(f, args) -> match List.assoc_opt f fn_arities with
          | Some a -> if a <> List.length args then raise BadProgram;
              List.iter (exp ctx) args
          | None -> raise BadProgram in
    let fn { name; args; body } = no_dups args; exp args body in
    List.iter fn fns; exp ["[input]"] main
end

module Eval = struct
  open AST

  type v = VBool of bool | VNum of int

  exception Type
  exception Overflow

  exception BreakExn of v (* for control flow *)

  let (+) a b =
    let sum = a + b in
    if a < 0 && b < 0 && sum >= 0 then raise Overflow
    else if a > 0 && b > 0 && sum <= 0 then raise Overflow
    else sum

  let negate x =
    if x = Int.min_int then raise Overflow else - x

  let (-) x y = x + negate y

  let ( * ) x y =
    let result = x * y in
    (* TODO: does this properly check for overflow? *)
    if x = 0 || result / x = y then result else raise Overflow

  let print = function
    | VBool b -> print_endline (if b then "true" else "false")
    | VNum x -> print_endline @@ string_of_int x

  let op1 o x = match o, x with
    | Add1, VNum x -> VNum (x+1)
    | Add1, VBool _ -> raise Type
    | Sub1, VNum x -> VNum (x-1)
    | Sub1, VBool _ -> raise Type
    | Isnum, VNum _ -> VBool true
    | Isnum, VBool _ -> VBool false
    | Isbool, VNum _ -> VBool false
    | Isbool, VBool _ -> VBool true
    | Print, v -> print v; v
    | Break, x -> raise (BreakExn x)

  let op2 o x y = match o, x, y with
    | Eq, VBool x, VBool y -> VBool (x = y)
    | Eq, VNum x, VNum y -> VBool (x = y)
    | Eq, _, _ -> raise Type
    | Add, VNum x, VNum y -> VNum (x + y)
    | Sub, VNum x, VNum y -> VNum (x - y)
    | Mul, VNum x, VNum y -> VNum (x * y)
    | Lt, VNum x, VNum y -> VBool (x < y)
    | Gt, VNum x, VNum y -> VBool (x > y)
    | Lte, VNum x, VNum y -> VBool (x <= y)
    | Gte, VNum x, VNum y -> VBool (x >= y)
    | _ -> raise Type

  let eval input defs =
    let rec go env = function
      | Num n -> VNum n
      | True -> VBool true
      | False -> VBool false
      | Input -> input
      | Var x -> !(List.assoc x env)
      | Let(binds, body) ->
          let env = List.fold_left
          (fun env (x,v) -> (x, ref (go env v))::env) env binds in
          go env body
      | Op1(op, x) -> op1 op (go env x)
      | Op2(op, x, y) -> (* OCaml eval order is fun *)
          let a = go env x in
          let b = go env y in
          op2 op a b
      | Set(var, v) ->
          let x = go env v in
          List.assoc var env := x; x
      | If(cond, t, e) -> begin
          match go env cond with
          | VBool false -> go env e
          | _ -> go env t (* Scheme is great, love how all numbers are truthy *)
          end
      | Block(ss, e) -> List.iter (fun b -> ignore @@ go env b) ss; go env e
      | Loop l -> begin
          try
            let rec loop () = ignore (go env l); loop () in loop ()
          with BreakExn v -> v end
      | Call(f, args) ->
          let f = List.find (fun x -> x.name = f) defs in
          let env = List.map2 (fun v a -> v, ref (go env a)) f.args args in
          go env f.body in
    go []
end

(* From stackoverflow *)
let read_file filename =
  let ch = open_in_bin filename in
  let s = really_input_string ch (in_channel_length ch) in
  close_in ch; s

let main () =
  let filename = Sys.argv.(1) in
  let contents = read_file filename in
  let input =
    let open Eval in
    if Array.length Sys.argv < 2 then VBool false else
    match Sys.argv.(2) with
    | "true" -> VBool true
    | "false" -> VBool false
    | x -> VNum (int_of_string x) in
  let s = SExpr.parse contents in
  let defs, main = AST.sexp_to_ast s in
  AST.check_program defs main;
  Eval.(print @@ eval input defs main)

let () = main ()

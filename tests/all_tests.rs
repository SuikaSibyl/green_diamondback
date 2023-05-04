mod infra;

// Your tests go here!
success_tests! {
    // Number and Boolean Literals
    {
        name: adder_num,
        file: "adder_num.snek",
        expected: "644",
    },
    {
        name: cobra_false_val,
        file: "cobra_false_val.snek",
        expected: "false",
    },

    // Input Expression
    {
        name: cobra_input_default,
        file: "cobra_input0.snek",
        expected: "false",
    },
    {
        name: cobra_input_bool,
        file: "cobra_input0.snek",
        input: "true",
        expected: "true",
    },
    {
        name: cobra_input_num,
        file: "cobra_input0.snek",
        input: "123",
        expected: "123",
    },

    // Simple Number Expressions
    {
        name: adder_add1,
        file: "adder_add1.snek",
        expected: "73",
    },
    {
        name: adder_add1_sub1,
        file: "adder_add1_sub1.snek",
        expected: "4",
    },
    {
        name: cobra_add_num,
        file: "cobra_add.snek",
        input: "10",
        expected: "15",
    },

    // Nested Arithmetic Expressions
    {
        name: boa_nested_arith0,
        file: "boa_nested_arith0.snek",
        expected: "35",
    },
    {
        name: boa_nested_arith1,
        file: "boa_nested_arith1.snek",
        expected: "25",
    },
    {
        name: boa_nested_arith2,
        file: "boa_nested_arith2.snek",
        expected: "0",
    },
    {
        name: cobra_nested_arith3,
        file: "cobra_nested_arith3.snek",
        input: "8",
        expected: "1117",
    },
    {
        name: boa_nested_arith4,
        file: "boa_nested_arith4.snek",
        expected: "-1",
    },

    // Dynamic Type Checks with isnum/isbool
    {
        name: cobra_type_check_succ0,
        file: "cobra_isnum.snek",
        expected: "false",
    },
    {
        name: cobra_type_check_succ1,
        file: "cobra_isnum.snek",
        input: "547",
        expected: "true",
    },
    {
        name: cobra_type_check_succ2,
        file: "cobra_isnum.snek",
        input: "true",
        expected: "false",
    },
    {
        name: cobra_type_check_succ3,
        file: "cobra_isbool.snek",
        expected: "true",
    },
    {
        name: cobra_type_check_succ4,
        file: "cobra_isbool.snek",
        input: "689",
        expected: "false",
    },
    {
        name: cobra_type_check_succ5,
        file: "cobra_type_check_succ5.snek",
        expected: "true",
    },

    // Comparison Expressions
    {
        name: cobra_compare_expr_succ0,
        file: "cobra_compare_expr_succ0.snek",
        expected: "true",
    },

    {
        name: cobra_compare_expr_succ2,
        file: "cobra_compare_expr_succ2.snek",
        expected: "true",
    },

    // Let expressions
    {
        name: boa_binding0,
        file: "boa_binding0.snek",
        expected: "5",
    },
    {
        name: boa_binding1,
        file: "boa_binding1.snek",
        expected: "-5",
    },

    {
        name: boa_binding_expr,
        file: "boa_binding_expr.snek",
        expected: "1225",
    },
    {
        name: boa_binding_nested,
        file: "boa_binding_nested.snek",
        expected: "1",
    },

    {
        name: boa_binding_chain,
        file: "boa_binding_chain.snek",
        expected: "3",
    },
    {
        name: boa_binding_nested_chain,
        file: "boa_binding_nested_chain.snek",
        expected: "12",
    },

    // Let expressions with shadowing
    {
        name: boa_shadowed_binding_succ0,
        file: "boa_shadowed_binding_succ0.snek",
        expected: "100",
    },
    {
        name: boa_shadowed_binding_succ1,
        file: "boa_shadowed_binding_succ1.snek",
        expected: "7",
    },
    {
        name: boa_shadowed_binding_succ2,
        file: "boa_shadowed_binding_succ2.snek",
        expected: "150",
    },
    {
        name: boa_shadowed_binding_succ3,
        file: "boa_shadowed_binding_succ3.snek",
        expected: "5",
    },
    {
        name: boa_shadowed_binding_succ4,
        file: "boa_shadowed_binding_succ4.snek",
        expected: "18",
    },
    {
        name: boa_shadowed_binding_succ5,
        file: "boa_shadowed_binding_succ5.snek",
        expected: "5",
    },
    {
        name: boa_shadowed_binding_succ6,
        file: "boa_shadowed_binding_succ6.snek",
        expected: "3",
    },
    {
        name: cobra_shadowed_binding_succ7,
        file: "cobra_shadowed_binding_succ7.snek",
        expected: "200",
    },

    // Misc complex expressions with arithmetic and let bindings
    {
        name: boa_complex_expr,
        file: "boa_complex_expr.snek",
        expected: "6",
    },
    {
        name: boa_quick_brown_fox,
        file: "boa_quick_brown_fox.snek",
        expected: "-3776",
    },

    // If expressions
    {
        name: cobra_if_expr_succ0,
        file: "cobra_if_expr_succ0.snek",
        expected: "10",
    },
    {
        name: cobra_if_expr_succ1,
        file: "cobra_if_expr_input.snek",
        input: "635",
        expected: "20",
    },
    {
        name: cobra_if_expr_succ2,
        file: "cobra_if_expr_succ2.snek",
        expected: "8",
    },
    {
        name: cobra_if_expr_succ3,
        file: "cobra_if_expr_succ3.snek",
        expected: "7",
    },

    // Set expr
    {
        name: cobra_set_expr_succ0,
        file: "cobra_set_expr1.snek",
        expected: "true",
    },
    {
        name: cobra_set_expr_succ1,
        file: "cobra_set_expr2.snek",
        expected: "25",
    },
    {
        name: cobra_set_expr_succ2,
        file: "cobra_set_expr3.snek",
        input: "25",
        expected: "true",
    },
    {
        name: cobra_set_expr_succ3,
        file: "cobra_set_expr3.snek",
        input: "20",
        expected: "false",
    },

    {
        name: cobra_loop_expr_succ0,
        file: "cobra_loop_expr0.snek",
        input: "3",
        expected: "6",
    },
    {
        name: cobra_loop_expr_succ1,
        file: "cobra_loop_expr0.snek",
        input: "7",
        expected: "5040",
    },
    {
        name: cobra_loop_expr_succ2,
        file: "cobra_loop_expr1.snek",
        expected: "-6",
    },
    // Functions and calls
    {
        name: diamondback_odd_even1,
        file: "diamondback_fun_mutual_recursion.snek",
        input: "4",
        expected: "true",
    },
    {
        name: diamondback_odd_even2,
        file: "diamondback_fun_mutual_recursion.snek",
        input: "201",
        expected: "false",
    },
    {
        name: diamondback_fun_nested_call,
        file: "diamondback_fun_nested_call.snek",
        input: "20",
        expected: "500",
    },
    {
        name: diamondback_fun_two_args,
        file: "diamondback_fun_two_args.snek",
        expected: "25",
    },
    {
        name: diamondback_fun_many_args,
        file: "diamondback_fun_many_args.snek",
        input: "-1",
        expected: "4294967296",
    },
    {
        name: diamondback_fun_many_calls,
        file: "diamondback_fun_many_calls.snek",
        input: "5",
        expected: "0\n1\n2\n3\n4\n5\n0",
    },
    // Printing / Function Calls
    {
        name: diamondback_fun_many_prints,
        file: "diamondback_fun_many_calls.snek",
        input: "999",
        expected: "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29\n30\n31\n32\n33\n34\n35\n36\n37\n38\n39\n40\n41\n42\n43\n44\n45\n46\n47\n48\n49\n50\n51\n52\n53\n54\n55\n56\n57\n58\n59\n60\n61\n62\n63\n64\n65\n66\n67\n68\n69\n70\n71\n72\n73\n74\n75\n76\n77\n78\n79\n80\n81\n82\n83\n84\n85\n86\n87\n88\n89\n90\n91\n92\n93\n94\n95\n96\n97\n98\n99\n100\n101\n102\n103\n104\n105\n106\n107\n108\n109\n110\n111\n112\n113\n114\n115\n116\n117\n118\n119\n120\n121\n122\n123\n124\n125\n126\n127\n128\n129\n130\n131\n132\n133\n134\n135\n136\n137\n138\n139\n140\n141\n142\n143\n144\n145\n146\n147\n148\n149\n150\n151\n152\n153\n154\n155\n156\n157\n158\n159\n160\n161\n162\n163\n164\n165\n166\n167\n168\n169\n170\n171\n172\n173\n174\n175\n176\n177\n178\n179\n180\n181\n182\n183\n184\n185\n186\n187\n188\n189\n190\n191\n192\n193\n194\n195\n196\n197\n198\n199\n200\n201\n202\n203\n204\n205\n206\n207\n208\n209\n210\n211\n212\n213\n214\n215\n216\n217\n218\n219\n220\n221\n222\n223\n224\n225\n226\n227\n228\n229\n230\n231\n232\n233\n234\n235\n236\n237\n238\n239\n240\n241\n242\n243\n244\n245\n246\n247\n248\n249\n250\n251\n252\n253\n254\n255\n256\n257\n258\n259\n260\n261\n262\n263\n264\n265\n266\n267\n268\n269\n270\n271\n272\n273\n274\n275\n276\n277\n278\n279\n280\n281\n282\n283\n284\n285\n286\n287\n288\n289\n290\n291\n292\n293\n294\n295\n296\n297\n298\n299\n300\n301\n302\n303\n304\n305\n306\n307\n308\n309\n310\n311\n312\n313\n314\n315\n316\n317\n318\n319\n320\n321\n322\n323\n324\n325\n326\n327\n328\n329\n330\n331\n332\n333\n334\n335\n336\n337\n338\n339\n340\n341\n342\n343\n344\n345\n346\n347\n348\n349\n350\n351\n352\n353\n354\n355\n356\n357\n358\n359\n360\n361\n362\n363\n364\n365\n366\n367\n368\n369\n370\n371\n372\n373\n374\n375\n376\n377\n378\n379\n380\n381\n382\n383\n384\n385\n386\n387\n388\n389\n390\n391\n392\n393\n394\n395\n396\n397\n398\n399\n400\n401\n402\n403\n404\n405\n406\n407\n408\n409\n410\n411\n412\n413\n414\n415\n416\n417\n418\n419\n420\n421\n422\n423\n424\n425\n426\n427\n428\n429\n430\n431\n432\n433\n434\n435\n436\n437\n438\n439\n440\n441\n442\n443\n444\n445\n446\n447\n448\n449\n450\n451\n452\n453\n454\n455\n456\n457\n458\n459\n460\n461\n462\n463\n464\n465\n466\n467\n468\n469\n470\n471\n472\n473\n474\n475\n476\n477\n478\n479\n480\n481\n482\n483\n484\n485\n486\n487\n488\n489\n490\n491\n492\n493\n494\n495\n496\n497\n498\n499\n500\n501\n502\n503\n504\n505\n506\n507\n508\n509\n510\n511\n512\n513\n514\n515\n516\n517\n518\n519\n520\n521\n522\n523\n524\n525\n526\n527\n528\n529\n530\n531\n532\n533\n534\n535\n536\n537\n538\n539\n540\n541\n542\n543\n544\n545\n546\n547\n548\n549\n550\n551\n552\n553\n554\n555\n556\n557\n558\n559\n560\n561\n562\n563\n564\n565\n566\n567\n568\n569\n570\n571\n572\n573\n574\n575\n576\n577\n578\n579\n580\n581\n582\n583\n584\n585\n586\n587\n588\n589\n590\n591\n592\n593\n594\n595\n596\n597\n598\n599\n600\n601\n602\n603\n604\n605\n606\n607\n608\n609\n610\n611\n612\n613\n614\n615\n616\n617\n618\n619\n620\n621\n622\n623\n624\n625\n626\n627\n628\n629\n630\n631\n632\n633\n634\n635\n636\n637\n638\n639\n640\n641\n642\n643\n644\n645\n646\n647\n648\n649\n650\n651\n652\n653\n654\n655\n656\n657\n658\n659\n660\n661\n662\n663\n664\n665\n666\n667\n668\n669\n670\n671\n672\n673\n674\n675\n676\n677\n678\n679\n680\n681\n682\n683\n684\n685\n686\n687\n688\n689\n690\n691\n692\n693\n694\n695\n696\n697\n698\n699\n700\n701\n702\n703\n704\n705\n706\n707\n708\n709\n710\n711\n712\n713\n714\n715\n716\n717\n718\n719\n720\n721\n722\n723\n724\n725\n726\n727\n728\n729\n730\n731\n732\n733\n734\n735\n736\n737\n738\n739\n740\n741\n742\n743\n744\n745\n746\n747\n748\n749\n750\n751\n752\n753\n754\n755\n756\n757\n758\n759\n760\n761\n762\n763\n764\n765\n766\n767\n768\n769\n770\n771\n772\n773\n774\n775\n776\n777\n778\n779\n780\n781\n782\n783\n784\n785\n786\n787\n788\n789\n790\n791\n792\n793\n794\n795\n796\n797\n798\n799\n800\n801\n802\n803\n804\n805\n806\n807\n808\n809\n810\n811\n812\n813\n814\n815\n816\n817\n818\n819\n820\n821\n822\n823\n824\n825\n826\n827\n828\n829\n830\n831\n832\n833\n834\n835\n836\n837\n838\n839\n840\n841\n842\n843\n844\n845\n846\n847\n848\n849\n850\n851\n852\n853\n854\n855\n856\n857\n858\n859\n860\n861\n862\n863\n864\n865\n866\n867\n868\n869\n870\n871\n872\n873\n874\n875\n876\n877\n878\n879\n880\n881\n882\n883\n884\n885\n886\n887\n888\n889\n890\n891\n892\n893\n894\n895\n896\n897\n898\n899\n900\n901\n902\n903\n904\n905\n906\n907\n908\n909\n910\n911\n912\n913\n914\n915\n916\n917\n918\n919\n920\n921\n922\n923\n924\n925\n926\n927\n928\n929\n930\n931\n932\n933\n934\n935\n936\n937\n938\n939\n940\n941\n942\n943\n944\n945\n946\n947\n948\n949\n950\n951\n952\n953\n954\n955\n956\n957\n958\n959\n960\n961\n962\n963\n964\n965\n966\n967\n968\n969\n970\n971\n972\n973\n974\n975\n976\n977\n978\n979\n980\n981\n982\n983\n984\n985\n986\n987\n988\n989\n990\n991\n992\n993\n994\n995\n996\n997\n998\n999\n0",
    },
    {
        name: diamondback_fun_no_args,
        file: "diamondback_fun_no_args.snek",
        expected: "true\nfalse\n0\n-1\n1",
    },
    {
        name: diamondback_calling_chain0,
        file: "diamondback_calling_chain0.snek",
        expected: "100\n100",
    },
    {
        name: diamondback_calling_chain1,
        file: "diamondback_calling_chain1.snek",
        expected: "100\n100",
    },
    {
        name: diamondback_conveyer_belt,
        file: "diamondback_conveyer_belt.snek",
        expected: "-50"
    },
    {
        name: diamondback_decreasing_args,
        file: "diamondback_decreasing_args.snek",
        expected: "12"
    },
    {
        name: diamondback_many_unused_functions,
        file: "diamondback_many_unused_functions.snek",
        input: "42",
        expected: "84",
    },
    {
        name: diamondback_namespaces,
        file: "diamondback_namespaces.snek",
        expected: "2"
    },
    {
        name: diamondback_namespaces,
        file: "diamondback_namespaces.snek",
        expected: "2"
    },

    // More complex recursive functions
    {
        name: diamondback_recursive_ackermann,
        file: "diamondback_recursive_ackermann.snek",
        expected: "61",
    },
    {
        name: diamondback_recursive_factorial,
        file: "diamondback_recursive_factorial.snek",
        expected: "1\n1\n2\n6\n24\n120\n720\n5040",
    },
    {
        name: diamondback_recursive_fibonacci,
        file: "diamondback_recursive_fibonacci.snek",
        expected: "55",
    },
}

runtime_error_tests! {
    // integer overflow
    {
        name: cobra_number_overflow_fail0,
        file: "cobra_number_overflow_fail0.snek",
        expected: "overflow",
    },
    {
        name: cobra_number_overflow_fail1,
        file: "cobra_number_overflow_fail1.snek",
        expected: "overflow",
    },
    {
        name: cobra_number_overflow_fail2,
        file: "cobra_add.snek",
        input: "4611686018427387899",
        expected: "overflow",
    },
    {
        name: cobra_number_overflow_fail3,
        file: "cobra_nested_arith3.snek",
        input: "4611686018427387890",
        expected: "overflow",
    },
    {
        name: diamondback_eventually_overflows,
        file: "diamondback_eventually_overflows.snek",
        expected: "overflow",
    },

    // type mismatch
    {
        name: cobra_invalid_argument_fail0,
        file: "cobra_invalid_argument_fail0.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail1,
        file: "cobra_invalid_argument_fail1.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail2,
        file: "cobra_invalid_argument_fail2.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail3,
        file: "cobra_invalid_argument_fail3.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail4,
        file: "cobra_invalid_argument_fail4.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail5,
        file: "cobra_invalid_argument_fail5.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail6,
        file: "cobra_invalid_argument_fail6.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail7,
        file: "cobra_nested_arith3.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail8,
        file: "cobra_if_expr_input.snek",
        input: "665",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail9,
        file: "cobra_set_expr3.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail10,
        file: "cobra_loop_expr0.snek",
        input: "5",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail11,
        file: "cobra_invalid_argument_fail11.snek",
        expected: "invalid argument",
    },
}

static_error_tests! {

    // Invalid S-expressions
    {
        name: boa_parse_sexp_fail1,
        file: "boa_parse_sexp_fail1.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_sexp_fail2,
        file: "boa_parse_sexp_fail2.snek",
        expected: "Invalid",
    },

    // Invalid tokens/operators
    {
        name: boa_parse_token_fail1,
        file: "boa_parse_token_fail1.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_token_fail2,
        file: "boa_parse_token_fail2.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_token_fail3,
        file: "boa_parse_token_fail3.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_token_fail4,
        file: "boa_parse_token_fail4.snek",
        expected: "Invalid",
    },


    // Invalid/Out of bounds Number Literal
    {
        name: cobra_number_bounds_fail0,
        file: "cobra_number_bounds_fail0.snek",
        expected: "Invalid",
    },
    {
        name: cobra_number_bounds_fail1,
        file: "cobra_number_bounds_fail1.snek",
        expected: "Invalid",
    },

    // Invalid operator arguments
    {
        name: boa_parse_op_fail1,
        file: "boa_parse_op_fail1.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_op_fail2,
        file: "boa_parse_op_fail2.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_op_fail3,
        file: "boa_parse_op_fail3.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_op_fai4,
        file: "boa_parse_op_fail4.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_op_fail5,
        file: "boa_parse_op_fail5.snek",
        expected: "Invalid",
    },
    {
        name: cobra_parse_op_fail6,
        file: "cobra_parse_op_fail6.snek",
        expected: "Invalid",
    },
    {
        name: cobra_parse_op_fail7,
        file: "cobra_parse_op_fail7.snek",
        expected: "Invalid",
    },
    {
        name: cobra_parse_op_fail8,
        file: "cobra_parse_op_fail8.snek",
        expected: "Invalid",
    },

    // Invalid let expressions
    {
        name: boa_parse_let_nobindings_fail,
        file: "boa_parse_let_nobindings_fail.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_let_improperargs_fail1,
        file: "boa_parse_let_improperargs_fail1.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_let_improperargs_fail2,
        file: "boa_parse_let_improperargs_fail2.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_let_improperargs_fail3,
        file: "boa_parse_let_improperargs_fail3.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_let_improperargs_fail4,
        file: "boa_parse_let_improperargs_fail4.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_let_improperargs_fail5,
        file: "boa_parse_let_improperargs_fail5.snek",
        expected: "keyword",
    },

    {
        name: boa_duplicate_binding_fail0,
        file: "boa_duplicate_binding_fail0.snek",
        expected: "Duplicate binding",
    },
    {
        name: boa_duplicate_binding_fail1,
        file: "boa_duplicate_binding_fail1.snek",
        expected: "Duplicate binding",
    },
    {
        name: boa_duplicate_binding_fail2,
        file: "boa_duplicate_binding_fail2.snek",
        expected: "Duplicate binding",
    },

    // Invalid if expressions
    {
        name: cobra_parse_if_fail0,
        file: "cobra_parse_if_fail0.snek",
        expected: "Invalid",
    },
    {
        name: cobra_parse_if_fail1,
        file: "cobra_parse_if_fail1.snek",
        expected: "Invalid",
    },

    // Unbound identifier
    {
        name: boa_unbound_identifier_fail0,
        file: "boa_unbound_identifier_fail0.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: boa_unbound_identifier_fail1,
        file: "boa_unbound_identifier_fail1.snek",
        expected: "Unbound variable identifier y",
    },
    {
        name: boa_unbound_identifier_fail2,
        file: "boa_unbound_identifier_fail2.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: cobra_unbound_identifier_fail3,
        file: "cobra_unbound_identifier_fail3.snek",
        expected: "Unbound variable identifier z",
    },
    {
        name: cobra_unbound_identifier_fail4,
        file: "cobra_unbound_identifier_fail4.snek",
        expected: "Unbound variable identifier t",
    },
    {
        name: cobra_unbound_identifier_fail5,
        file: "cobra_unbound_identifier_fail5.snek",
        expected: "Unbound variable identifier x",
    },

    // Invalid block
    {
        name: cobra_parse_block_fail0,
        file: "cobra_parse_block_fail0.snek",
        expected: "Invalid",
    },

    // Invalid break
    {
        name: cobra_invalid_break_fail0,
        file: "cobra_invalid_break_fail0.snek",
        expected: "break",
    },

    // Invalid loop
    {
        name: cobra_invalid_loop_fail0,
        file: "cobra_invalid_loop_fail0.snek",
        expected: "Invalid",
    },
    // Invalid function
    {
        name: diamondback_fun_duplicate_parameters_fail0,
        file: "diamondback_fun_duplicate_parameters_fail0.snek",
        expected: "",
    },
    {
        name: diamondback_fun_duplicate_parameters_fail1,
        file: "diamondback_fun_duplicate_parameters_fail1.snek",
        expected: "",
    },
    {
        name: diamondback_fun_input_fail0,
        file: "diamondback_fun_input_fail0.snek",
        expected: "",
    },
    {
        name: diamondback_fun_input_fail1,
        file: "diamondback_fun_input_fail1.snek",
        expected: "",
    },
    {
        name: diamondback_fun_not_exists_fail,
        file: "diamondback_fun_not_exists_fail.snek",
        expected: "",
    },
    {
        name: diamondback_fun_wrong_numargs_fail,
        file: "diamondback_fun_wrong_numargs_fail.snek",
        expected: "",
    },
    {
        name: diamondback_fun_duplicate_names_fail,
        file: "diamondback_fun_duplicate_names_fail.snek",
        expected: "",
    },

    {
        name: diamondback_not_fun_fail0,
        file: "diamondback_not_fun_fail0.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail1,
        file: "diamondback_not_fun_fail1.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail2,
        file: "diamondback_not_fun_fail2.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail3,
        file: "diamondback_not_fun_fail3.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail4,
        file: "diamondback_not_fun_fail4.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail5,
        file: "diamondback_not_fun_fail5.snek",
        expected: "",
    },

    {
        name: diamondback_no_expr_fail,
        file: "diamondback_no_expr_fail.snek",
        expected: "",
    },
    {
        name: diamondback_nested_fun_fail,
        file: "diamondback_nested_fun_fail.snek",
        expected: "",
    },

    {
        name: diamondback_fun_scope_fail0,
        file: "diamondback_fun_scope_fail0.snek",
        expected: "",
    },
    {
        name: diamondback_fun_scope_fail1,
        file: "diamondback_fun_scope_fail1.snek",
        expected: "",
    },
    {
        name: diamondback_fun_scope_fail2,
        file: "diamondback_fun_scope_fail2.snek",
        expected: "",
    },
    {
        name: diamondback_fun_scope_fail3,
        file: "diamondback_fun_scope_fail3.snek",
        expected: "",
    },

    {
        name: diamondback_function_is_keyword_fail,
        file: "diamondback_function_is_keyword_fail.snek",
        expected: "",
    },
    {
        name: diamondback_function_arg_is_keyword_fail,
        file: "diamondback_function_arg_is_keyword_fail.snek",
        expected: "",
    },
}

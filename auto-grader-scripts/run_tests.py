import subprocess
import re
import json
from pathlib import Path
import argparse

# Timeout in seconds
TIMEOUT = 300


def run_tests():
    # Only run integration tests
    timedout = False
    try:
        result = subprocess.run(['cargo', 'test', '--test', 'all_tests'],
                                timeout=TIMEOUT,
                                stdout=subprocess.PIPE)
        stdout = result.stdout.decode()
    except subprocess.TimeoutExpired as exc:
        stdout = exc.stdout.decode()
        timedout = True
        pass
    print(stdout)

    pattern = re.compile(r'test (\w+) ... (ok|FAILED|ignored)')
    results = []
    for line in stdout.splitlines():
        match = pattern.match(line)
        if match:
            results.append({
                'name': match.group(1),
                'pass': match.group(2) == 'ok',
                'output': line.rstrip()
            })
    return (timedout, results)


def gradescope_format(timedout, results):
    tests = []
    for test in results:
        tests.append({
            'score': 1 if test['pass'] else 0,
            'max_score': 1,
            'name': test['name'],
            'output': test['output']
        })
    if timedout:
        output = f'Your submission timed out after {TIMEOUT} seconds. The results may be incomplete.'
    else:
        output = 'The autograded score is not an official grade'
    return {'output': output, 'stdout_visibility': 'visible', 'tests': tests}


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("results_path", type=Path)
    args = parser.parse_args()

    (timedout, results) = run_tests()
    gradescope = gradescope_format(timedout, results)
    args.results_path.parent.mkdir(parents=True, exist_ok=True)
    with open(args.results_path, 'w') as f:
        f.write(json.dumps(gradescope))

import subprocess
import re
import json
from pathlib import Path
import argparse


def run_tests():
    # Only run integration tests
    result = subprocess.run(['cargo', 'test', '--test', 'all_tests'],
                            stdout=subprocess.PIPE)
    stdout = result.stdout.decode()
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
    return results


def gradescope_format(results):
    tests = []
    for test in results:
        tests.append({
            'score': 1 if test['pass'] else 0,
            'max_score': 1,
            'name': test['name'],
            'output': test['output']
        })
    return {
        'output': 'The autograded score is not an official grade',
        'stdout_visibility': 'visible',
        'tests': tests
    }


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("results_path", type=Path)
    args = parser.parse_args()

    results = run_tests()
    gradescope = gradescope_format(results)
    args.results_path.parent.mkdir(parents=True, exist_ok=True)
    with open(args.results_path, 'w') as f:
        f.write(json.dumps(gradescope))

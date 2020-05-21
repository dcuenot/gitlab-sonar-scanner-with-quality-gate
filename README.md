# CLI to get Qualtiy Gate of SonarQube

[![Continuous integration](https://github.com/dcuenot/sonar-qg-in-cli/workflows/Continuous%20integration/badge.svg)](https://github.com/dcuenot/sonar-qg-in-cli/actions?query=workflow%3A%22Continuous+integration%22)
[![Security audit](https://github.com/dcuenot/sonar-qg-in-cli/workflows/Security%20audit/badge.svg)](https://github.com/dcuenot/sonar-qg-in-cli/actions?query=workflow%3A%22Security+audit%22)
[![codecov](https://codecov.io/gh/dcuenot/sonar-qg-in-cli/branch/master/graph/badge.svg)](https://codecov.io/gh/dcuenot/sonar-qg-in-cli)


# CLI description
```
Sonar Quality Gate CLI 1.0.0

USAGE:
    sonar_qg [FLAGS] [OPTIONS] [report-task-path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)

OPTIONS:
    -g, --gitlab_private_token <gitlab-private-token>     [env: GITLAB_PRIVATE_TOKEN=]

ARGS:
    <report-task-path>     [default: .scannerwork/report-task.txt]

```

# Sample of output
![Output](docs/cli_result.png?raw=true "CLI Output")
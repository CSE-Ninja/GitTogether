# GitTogether

GitTogether is a tool that automatically pulls the contributor statistics for your repository and visualize it in a SVG file.

# Contributors

![](https://activity-action.vercel.app/api/handler?repo=aoli-al/activity-action&)


# Instructions

## Installation

To build and install from source:

```bash
cargo install --path .
```

## Usage

```shell
gittogether [OPTIONS] <repo> [period]

Arguments:
  <repo>                 owner/repo, e.g. rust-lang/rust
  [period]               Legacy period string: "Name/START/END;Name2/START/END;..."

Options:
  -s, --style <style>    Render style (default: compact)
  -o, --output <path>    Output SVG path (default: image.svg)
  -c, --config <file>    YAML config file with periods (takes precedence over [period])
  -h, --help             Show help
  -V, --version          Show version
```

### Input Format: YAML (recommended)

Create a YAML configuration file (e.g., `.gittogether.yml`) similar to the example below.

```yaml
periods:
- name: "2020"
  start: 2020-01-01T00:00:00Z
  end: 2020-12-31T23:59:59Z

- name: "2021"
  start: 2021-01-01T00:00:00Z
  end: 2021-12-31T23:59:59Z

- name: "2022"
  start: 2022-01-01T00:00:00Z
  end: 2022-12-31T23:59:59Z

- name: "2023"
  start: 2023-01-01T00:00:00Z
  end: 2023-12-31T23:59:59Z

- name: "2024"
  start: 2024-01-01T00:00:00Z
  end: 2024-12-31T23:59:59Z

- name: "2025"
  start: 2025-01-01T00:00:00Z
  end: 2025-12-31T23:59:59Z
```

Then execute the following:

```shell
gittogether owner/repo --config .gittogether.yml --style compact -o activity.svg
```

**Important: Dates must be RFC 3339 (e.g., YYYY-MM-DDTHH:MM:SSZ). They deserialize directly into chrono::DateTime<Utc>.**

### Input Format: Legacy CLI String (recommended)

```shell
gittogether owner/repo \
  "Project 1/2020-01-01T00:00:00Z/2020-12-31T23:59:59Z;\
   Sprint 42/2021-02-01T00:00:00Z/2021-02-28T23:59:59Z" \
  --style compact -o activity.svg
```

**Format:** `Name/START/END;Name2/START/END;...`

GitTogether will parse each START/END as RFC 3339.

### Input Format: Default / Unspecified

If neither a YAML config file or legacy CLI string is provided, GitTogether will default to showing the last 30 days of activity for the repository.

### Output

GitTogether will output a single SVG for the given repository that summarizes all contributions for each of the specified time periods.

* Use `--output` to change the filename or path (default: `image.svg`)
* Use `--style` to change the style used by the SVG

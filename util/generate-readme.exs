separator = "<!-- usage -->"

path = Path.join(File.cwd!(), "README.md")

[readme, _] =
  path
  |> File.read!()
  |> String.split(separator)

{usage, 0} = System.cmd("cargo", ["run", "--", "--help"])

usage = """

## Usage

```
#{usage}
```
"""

readme = readme <> separator <> usage

File.write!(path, readme)

IO.puts("")
IO.puts("README.md amended")
System.halt()

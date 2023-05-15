import sys

from assembly import Assembly
from project import Project

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: export.py <project|assembly>")
        sys.exit(1)
    if sys.argv[1] == "project":
        print(Project.schema_json(indent=2))
    elif sys.argv[1] == "assembly":
        print(Assembly.schema_json(indent=2))
    else:
        print(f"Unknown command: {sys.argv[1]}")
        sys.exit(1)

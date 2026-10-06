"""Reproduce #42 without plotting dependencies: python3 experiments/reproduce_report_ratio.py."""
import ast
from pathlib import Path

path = Path(__file__).resolve().parents[1] / 'rust/out.py'
tree = ast.parse(path.read_text())
legacy = next((node for node in ast.walk(tree) if isinstance(node, ast.FunctionDef) and node.name == 'annotate'), None)
if legacy is not None:
    namespace = {'min_psql': 100}
    exec(compile(ast.fix_missing_locations(ast.Module(body=[legacy], type_ignores=[])), str(path), 'exec'), namespace)
    result = namespace['annotate'](200)
else:
    import sys
    sys.path.insert(0, str(path.parents[1] / 'scripts'))
    from benchmark_report import comparison
    result = comparison((200, 0), (100, 0))
print(result)
assert 'slower' in result, 'Doublets takes twice as long, so it must be labelled slower'

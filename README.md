# how to publish new version

1. build executable
```bash
cargo build --release
```
2. move releasable to python package
3. create source distribution
```bash
cd python-package
python3 setup.py sdist
```
5. test everything works with test pypi
```bash
twine upload --repository testpypi dist/*
pip install --index-url https://test.pypi.org/simple/beautiful_pytest
```
6. upload package using twine and check it worked
```bash
cd python-package
twine upload dist/*
pip install beautiful_pytest
```

from setuptools import setup, find_packages

setup(
    name='beautiful_pytest',
    version='0.1.0',
    packages=find_packages(),
    include_package_data=True,
    package_data={
        'beautiful_pytest': ['build/rust_app'],
    },
    entry_points={
        'console_scripts': [
            'beautiful_pytest=beautiful_pytest.wrapper:main',
        ],
    },
)

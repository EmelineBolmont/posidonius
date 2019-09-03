# Posidonius

[Posidonius](http://www.blancocuaresma.com/s/) is a N-body code based on the tidal model used in [Mercury-T]((http://www.emelinebolmont.com/)) ([Bolmont et al. 2015](http://adsabs.harvard.edu/abs/2015A%26A...583A.116B)). It uses a symplectic integrator (WHFast, [Rein & Tamayo 2015](http://adsabs.harvard.edu/abs/2015MNRAS.452..376R)) to compute the evolution of positions and velocities, which is also combined with a midpoint integrator to calculate the spin evolution in a consistent way. As Mercury-T, Posidonius takes into account tidal forces, rotational-flattening effects and general relativity corrections. It also includes different evolution models for FGKML stars and gaseous planets.

The N-Body code is written in Rust ([Blanco-Cuaresma et al. 2017](http://adsabs.harvard.edu/abs/2017IAUS..325..341B)) and a python package is provided to easily define simulation cases in JSON format, which is readable by the Posidonius integrator.

## Requirements

- Rust: [see rustup](https://www.rustup.rs/)
- Python: [see anaconda](https://www.anaconda.com/download/)

## Installation

First, install the N-body simulator by running from the current directory:

```bash
cargo install --path . --force
```

The executable will be copied into `$HOME/.cargo/bin/`. Then, install the python package to create cases by running:

```bash
curl -O https://www.cfa.harvard.edu/~sblancoc/posidonius/input.tar.gz
tar -zxvf input.tar.gz && rm -f input.tar.gz
python setup.py install --user
```

The `--user` flag will install the package in your `$HOME/.local/lib/python2.7/site-packages/`.

Both tools can be uninstalled by executing:

```
cargo uninstall posidonius
python setup.py install --user --record files.txt
cat files.txt | xargs rm -rf && rm -f files.txt
```

## Usage

### Create a JSON case

The user can design his/her own simulation with a python script, which will create a JSON file with the simulation description. This file can be read later on by Posidonius to start the simulation. The code includes several examples in the cases directory, and they can be executed by running:

```bash
python cases/Bolmont_et_al_2015/case3.py target/case3.json
python cases/Bolmont_et_al_2015/case4.py target/case4.json
python cases/Bolmont_et_al_2015/case7.py target/case7.json
python cases/example.py target/example.json
```

### Start the simulation of a JSON case

The simulations can be started using JSON files (which describe the initial conditions). When starting a simulation, the recovery and historic snapshot file names should be specified. The former will contain the information needed to resume interrupted simulations, while he latter stores the evolution of the simulation over the years.

```bash
posidonius start target/case3.json target/case3.bin target/case3_history.bin
posidonius start target/case4.json target/case4.bin target/case4_history.bin
posidonius start target/case7.json target/case7.bin target/case7_history.bin
posidonius start target/example.json target/example.bin target/example_history.bin
```

The flag `--silent` can be added to avoid printing the current year of the simulation.

### Resume an interrupted simulation

Interrupted simulations can be restored using the recovery snapshot file. The historic snapshot filename has to be specified also to continue storing the history of the simulation.

```bash
posidonius resume target/case3.bin target/case3_history.bin
posidonius resume target/case4.bin target/case4_history.bin
posidonius resume target/case7.bin target/case7_history.bin
posidonius resume target/example.bin target/example_history.bin
```

The flag `--silent` can be added to avoid printing the current year of the simulation.

### Analyse a simulation

While a simulation is in progress or when it has ended, the historic snapshot file can be interpreted to generate a plain text file and a plot with the history of the simulation:

```bash
python scripts/explore_history.py target/case3.json   target/case3_history.bin
python scripts/explore_history.py target/case4.json   target/case4_history.bin
python scripts/explore_history.py target/case7.json   target/case7_history.bin
python scripts/explore_history.py target/example.json target/example_history.bin
```

To explore what possible resonances might be present in the system:

```
python scripts/explore_timed_resonances.py target/case3_history.bin
python scripts/explore_timed_resonances.py target/case4_history.bin
python scripts/explore_timed_resonances.py target/case7_history.bin
python scripts/explore_timed_resonances.py target/example_history.bin
```

Finally, to study a given resonance (e.g., 3:2) between planet one and two:

```
python scripts/explore_single_resonance.py target/case3_history.bin 1 2 3 2
python scripts/explore_single_resonance.py target/case4_history.bin 1 2 3 2
python scripts/explore_single_resonance.py target/case7_history.bin 1 2 3 2
python scripts/explore_single_resonance.py target/example_history.bin 1 2 3 2
```


## Testing

This section is intended only for developers. 

### Python

Prepare a python environment:

```
virtualenv python
source python/bin/activate
pip install -r requirements.txt
pip install -r dev-requirements.txt
```

Run the python tests:

```
pytest
```

If tests fail, it is because the current posidonius version generate JSON files that differ from the previous version. The error will contain the names of the files that differ so that the developer can compare them:

```
vimdiff posidonius/tests/data/test_tides-enabled_tides/case.json posidonius/tests/data/tmp/test_tides-enabled_tides/case.json
```

If the new file (in the `tmp` directory) is the expected one given the changes done to the code, then the `case.json` files could be deleted and recreated by running the test again:

```
find posidonius/tests/data/ -name 'case.json' -delete
pytest
```

### Rust

Run the rust tests:

```
cargo tests
```

#### Rust

If the test results for a case differ from previous executions and the difference is justified and expected by a change in the code, then all the files `particle_*.json` can be erased and recreated:

```
find tests/data/ -name 'particle_*.json' -delete
cargo tests
```

But if the difference is not expected, then the changes made to the code have affected the expected behavior of posidonius and the changes you have made to your code to identify the reason/bug.


#### Rust vs Python

If the results when using a JSON generated with rust differ from the results when using a JSON generated by python, the first thing to check is if there are differences between the JSON files:

```
vimdiff tests/data/test_tides-enabled_tides/case.json posidonius/tests/data/test_tides-enabled_tides/case.json
```

If it is necessary to recreate all the rust test's `case.json`, it is enough to delete them all, run the test again and run the cleaning script (re-format the JSON files to make them easily comparable to the python generated ones):

```
find tests/data/ -name 'case.json' -delete
cargo tests
python scripts/clean_json.py
```

If the results still differ, it is necessary to verify the changes you have made to your code to identify the reason/bug.

#### Benchmarks

We can use benchmarks to assess the relative performance of code changes (i.e., statistically significant variations in execution time / speed).  First, we need a `reference` measurement before we make any changes:

```
rm -rf target/criterion/
cargo bench -- --save-baseline reference
```

Second, we can apply our changes to the code. And finally, we can run the benchmarks again to compare it to the previous `reference` measurement:

```
cargo bench -- --baseline reference
```

If we need to execute only some of the benchmarks, they can be filtered by their name:

```
cargo bench -- --baseline reference calculate_additional_effects/all
```

If [gnuplot](http://www.gnuplot.info/) is installed in the system, a report with plots will be generated/updated after each execution:

```
open target/criterion/report/index.html
```


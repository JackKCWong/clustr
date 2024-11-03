# clustr

Use text metrics to find similar strings clusters. Given `n` strings in a csv, it run the metric function against every pair of them and output the metric if it's above the given threshold.


# usage

```
# example: read strings from the 2nd field of the test.csv and compare them using jaro and output pairs when their similarity is above 0.99

clustr -m jaro -t 0.99 tests/test.csv 1

Usage: clustr [OPTIONS] [INPUT_FILE] [INDEX]

Arguments:
  [INPUT_FILE]  Optional file to operate on
  [INDEX]       Optional field index to operate on

Options:
  -m, --metric <METRIC>        text metric to use.
                               available: hamming, levenshtein, normalized_levenshtein, osa_distance,
                                        damerau_levenshtein, normalized_damerau_levenshtein, jaro,
                                        jaro_winkler, sorensen_dice [default: jaro_winkler]
  -t, --threshold <THRESHOLD>  metric threshold, only return matches above this value [default: 0]
      --debug                  debug output
  -h, --help                   Print help
  -V, --version                Print version
```

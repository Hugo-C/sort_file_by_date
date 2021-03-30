## This is a small script to sort files having a timestamp in their name to ordered files

This is mainly a pet project to learn rust, although it is applied on a real need.

On the example applied, it took less than 2s to sort ~75k files.
___
Hierarchy of the files:

```
- root directory (correspond to the first argument of this program)  
\ - 2019  
\ - 2020
  \ - 01
    \ - file_to_sort_<timestamp>.json
    \ - ...
  \ - 02
  \ - ...  
```

The program is applied to a given year at a time, but threads are used to parralelize work on the subdirectories (corresponding to each month, from 01 to 12)

The result are the following changes in place:
```
- root directory (correspond to the first argument of this program)  
\ - 2019  
\ - 2020
  \ - 01
    \ - 0.json
    \ - 1.json
    \ - 2.json
    \ - ...
  \ - 02
  \ - ...  
```
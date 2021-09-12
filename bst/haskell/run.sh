#!/bin/bash

ghc main.hs 
./main > vis.dot 
dot vis.dot -Tpng -o vis.png

#!/bin/sh
git clone https://github.com/viperproject/silver-sif-extension
cd silver-sif-extension
git checkout meilers_plugin_rel
git clone https://github.com/viperproject/silver/
sbt compile


# power-law-playground

_Simple simulation in Rust that produces a power law distribution._

## Overview

This repo is just a small playground that was mainly inspired by a talk with my friend [konstantin-ogulchansky](https://github.com/konstantin-ogulchansky) about his [thesis](https://github.com/konstantin-ogulchansky/hypergraphs). The work is about hypergraphs where I barely understand any word, but what clicked me hard was the idea of [power law](https://en.wikipedia.org/wiki/Power_law) (you probably heard about [Pareto principle](https://en.wikipedia.org/wiki/Pareto_principle)). So I decided to create a very simple model that will be able to produce something that also follows power law distribution.

## Model

The basic metaphore of the model is economy. The rules are simple:
- Each _human_ in _population_ has some _money_
- At each simulation step:
  * A _random human_ receives an _income_
  * A _new human_ is born
  * A _random human_ dies

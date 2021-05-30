# power-law-playground

_Simple simulation in Rust that produces a power law distribution._

## Overview

This repo is just a small playground that was mainly inspired by a talk with my friend [konstantin-ogulchansky](https://github.com/konstantin-ogulchansky) about his [thesis](https://github.com/konstantin-ogulchansky/hypergraphs). The thesis is about hypergraphs where I barely understand any word, but what clicked me for a moment was the idea of [power law](https://en.wikipedia.org/wiki/Power_law) (you probably heard about [Pareto principle](https://en.wikipedia.org/wiki/Pareto_principle)). So I decided to create a very simple model that would be able to produce something that also follows a power law distribution.

## Model

The basic metaphor of the model is the goblins economy. Rules are simple:
- Each _goblin_ in _population_ has some _gold_
- At simulation step:
  * A _random goblin_ receives an _income_
  * A _new goblin_ is born
  * A _random goblin_ dies

What makes this thing interesting is that random is _weighted_: the more gold a goblin already has, the better chances he'll receive more.

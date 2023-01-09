# FSM Cpp

- `cd build`

- `make`

- `cd fsm`

- `./fsm` or `./fsm17`

## About FSM

[Source code](https://github.com/hbarcelos/cpp-state-machine)

The main idea of a FSM's implementation is consisted of a `Machine`, which holds data, state and event functions, and be a friend of `AbstractStates` in order to call the real event functions; an abstract class `AbstractStates`, which used for concrete states to be derived from; several concrete states who implement actual data and state changing.

## About FSM17 and FSM17EHN

[Source article](https://sii.pl/blog/en/implementing-a-state-machine-in-c17/?category=hard-development&tag=c17-en,embedded-competency-center-en,state-machine-en,stdvariant-en,templates-en)

- States require passing some arguments during initialization?

- Runtime event?

- Generate a visual description of the state machine?

[Source article part2](https://sii.pl/blog/en/implementing-a-state-machine-in-c17-part-2/)

![fsm](./fsm.png)

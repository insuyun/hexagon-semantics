# Stoke memory model summary

`src/symstate/memory` 참조

## Trivial

```
Memory models all reads as seeing a new unconstrained value.  Records
the address/value of all reads/writes.  Could, theoretically, be used to
build a complete memory model by adding constraints to the read values.
```

모든 write를 list로 관리.
모든 read의 경우 무조건 새로운 bitvector를 반환함.
일단 동작은 하는 memory 모델이되, write/read 연산 간의 상관관계는 구현되어있지 않음.

## Flat

```
Models memory as a giant array
```

Array를 통해서 구현한 memory.
Contraints 관리가 들어가있지만 정확히 어떤 의미인지는 아직 모름.

## Cell

```
Keeps track of a fixed, finite number of symbolic memory locations ("cells")
```

line_no를 통해 cell이라고 불리는 bitvector에 접근할 수 있는 구조로 설계.
앞서 설명된 Trivial/Flat과 같이 general한 memory 구조는 아님.

## Arm

```
Models memory as a giant array
```

Flat과 동일하지만 contraints에 대한 처리 구문이 들어가있음.
어떠한 구조로 이뤄지는지 쉽게 파악이 되지 않아서 시간이 필요함.
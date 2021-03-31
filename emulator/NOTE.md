# Note for implementation

- decbin instruction의 경우 P0에 write. 그러나 .new로 쓰일 수 없고, auto-and predicate의 대상도 되지 않음.
  P0에 dependency가 걸리기 때문에, 다른 P0에 write하는 instruction과의 dependency 고려가 필요하다.
  이런 다른 instruction이 있는지 살펴볼 필요 있음.
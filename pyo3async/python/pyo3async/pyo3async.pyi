# @file:	pyo3async.pyi
# @author:	Jacob Xie
# @date:	2023/09/09 00:08:23 Saturday
# @brief:

from typing import Optional, List
from dataclasses import dataclass

# class PA(object):
#     def __init__(self, key: int, name: Optional[str], props: List[str]) -> None: ...
#     def to_json(self) -> str: ...
#     @property
#     def key(self) -> int: ...
#     @key.setter
#     def key(self, key: int) -> None: ...
#     @property
#     def name(self) -> Optional[str]: ...
#     @name.setter
#     def name(self, name: Optional[str]) -> None: ...
#     @property
#     def props(self) -> List[str]: ...
#     @props.setter
#     def props(self, props: List[str]) -> None: ...

@dataclass
class PA:
    key: int
    name: Optional[str]
    props: List[str]

    def to_json(self) -> str: ...

# sleep 1s
async def rust_sleep() -> None: ...

# log info
def rust_log() -> None: ...

# sleep Ns
async def rust_sleep_print(secs: int, value: PA) -> PA: ...

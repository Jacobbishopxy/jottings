# @file:	py_notify.py
# @author:	Jacob Xie
# @date:	2023/04/20 16:19:02 Thursday
# @brief:


import inotify.adapters
import datetime as dt


def main():
    i = inotify.adapters.Inotify()

    i.add_watch("./log")

    for event in i.event_gen(yield_nones=False):
        (_, type_names, path, filename) = event  # type: ignore

        msg = f"{dt.datetime.now()} PATH=[{path}] FILENAME=[{filename}] EVENT_TYPES={type_names}"
        print(msg)


if __name__ == "__main__":
    main()

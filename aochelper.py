# AoC Helper
# Easy download of puzzle inputs into text files

import glob
import os
import sqlite3
from shutil import copyfile

import requests
import click

MOZILLA_COOKIE_PATH = glob.glob(
    "/mnt/c/Users/*/AppData/Roaming/Mozilla/Firefox/Profiles/*default-release/cookies.sqlite"
)[0]

TEMPLATE_SRC = os.path.join("src", "day_template.rs")
MAIN_SRC = os.path.join("src", "main.rs")


def get_session_cookie() -> str:
    """Get login cookie from firefox."""
    # find mozilla cookie path:
    # copy the file locally
    # this prevents Sqlite I/O errors if firefox is running
    local_db = copyfile(MOZILLA_COOKIE_PATH, "cookies-tmp.sqlite")

    # connect to the database & execute the query
    db_connection = sqlite3.connect(local_db)
    query = "SELECT name, value FROM moz_cookies WHERE host='.adventofcode.com'"
    cursor = db_connection.cursor()
    cursor.execute(query)
    key, value = cursor.fetchone()

    # close the database, and delete the temporary file
    db_connection.close()
    os.remove(local_db)

    # check that we got what we thought we're getting
    assert key == "session"

    return f"{key}={value}"


def copy_template(day: str) -> os.PathLike:
    """Copy the template file as the next source file."""
    target_file = os.path.join("src", f"day_{day}.rs")
    if not os.path.exists(target_file):
        copyfile(TEMPLATE_SRC, target_file)
    return target_file


def add_mod_to_main(day: str):
    """Add the day to the match statement"""
    new_match_arm = f"        {day} => day_{day}::main(contents),\n"
    next_module = f"pub mod day_{day};\n"

    with open(MAIN_SRC, "r") as main_src:
        contents = main_src.readlines()

    if new_match_arm in contents or next_module in contents:
        return None

    for index, line in enumerate(contents):
        if line.strip() == "match config.day {":
            contents.insert(index + 1, new_match_arm)
            break

    with open(MAIN_SRC, "w") as main_src:
        main_src.write("".join(contents))
        main_src.write(next_module)


@click.command()
@click.argument("year")
@click.argument("day")
def get_input(year: str, day: str) -> os.PathLike:
    puzzle_url: str = f"https://adventofcode.com/{year}/day/{day}"
    input_url: str = puzzle_url + "/input"
    resp = requests.get(input_url, headers={"Cookie": get_session_cookie()})

    file_path: os.PathLike = os.path.join("inputs", f"{year}.{day}")
    match resp.status_code:
        case 200:  # OK
            with open(file_path, "w") as fp:
                fp.write(resp.text)
        case 400:
            raise Exception(f"Unauthorized: update cookie by logging in with Firefox.")
        case 404:
            raise Exception(f"Puzzle is unavailable.")
        case _:
            raise Exception(
                f"Could not get puzzle input: {resp.status_code} {resp.reason}"
            )

    copy_template(day)  # TODO: move to separate click function
    add_mod_to_main(day)  # TODO: move to separate click function
    return file_path


if __name__ == "__main__":
    get_input()

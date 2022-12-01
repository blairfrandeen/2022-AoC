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

    return file_path


if __name__ == "__main__":
    get_input()

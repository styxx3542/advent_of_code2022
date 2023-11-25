import os
import requests
import subprocess

COOKIE = "INSERT COOKIE HERE"


def download_file(url, directory, cookie):
    filename = "input.txt"
    filepath = os.path.join(directory, filename)

    if not os.path.exists(directory):
        subprocess.run(["cargo", "new", directory, "--vcs", "none"])

    if not os.path.exists(filepath):
        headers = {"Cookie": f"session={cookie}"}
        response = requests.get(url, headers=headers)

        if response.status_code == 200:
            with open(filepath, "wb") as file:
                file.write(response.content)
            print(f"File downloaded successfully to {filepath}")
        else:
            print("Failed to download file")
    else:
        print("File already exists")


for day in range(2, 26):
    day_a = "day" + str(day) + "a/"
    day_b = "day" + str(day) + "b/"
    url = f"https://adventofcode.com/2022/day/{day}/input"
    download_file(url, day_a, COOKIE)
    download_file(url, day_b, COOKIE)

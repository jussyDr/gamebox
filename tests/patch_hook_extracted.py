import glob
import os

for file_name in glob.glob(os.path.join(os.path.dirname(__file__), "files/**/*.Gbx") , recursive=True):
    with open(file_name, "r+b") as file:
        file.seek(7)
        body_compression = file.read(1)[0]

        if body_compression == 85:   
            file.seek(13)
            file.write(bytes([0, 0, 0, 0]))

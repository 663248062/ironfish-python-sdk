import pyironfishlib
import binascii


def main():
    fish_hash_context = pyironfishlib.fishhash.FishHashContext(False)
    header_data = "0000000000000000f432000000000000020c176b935dfecf0185571560ced81e0bbbd96fb32316c4192df2bea6ceec0c647dc1bf7ab71ed553b330af0baec871ba140b19cf54148b983a6f373b83e34bf6b3b6293d3b876733bd2963a9ee1a8c5e273cdfb6f6f5f4aa54e72500000000046c18593c658c623c36bd8686f8cce7ac916cb179844c3073183131189f25ba8501000068617368706f6f6c000000000000000000000000000000000000000000000000"
    hash = fish_hash_context.hash(bytes(binascii.a2b_hex(header_data)))
    print(binascii.b2a_hex(bytes(hash)))


if __name__ == "__main__":
    main()
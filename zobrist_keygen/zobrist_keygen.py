import secrets

# Define the dimensions of the Zobrist keys array
FILES = 8
RANKS = 8
PIECE_TYPES = 6
PLAYERS = 2

# Generate castling keys
castling_keys = [secrets.randbits(64) for _ in range(16)]

print("pub const CASTLING_KEYS: [u64; 16] = [")
print("    ", end="")
print(", ".join(f"0x{key:016X}" for key in castling_keys), end=",\n")
print("];")

# Generate en passant keys
en_passant_keys = [secrets.randbits(64) for _ in range(FILES)]

print("pub const EN_PASSANT_KEYS: [u64; 8] = [")
print("    ", end="")
print(", ".join(f"0x{key:016X}" for key in en_passant_keys), end=",\n")
print("];")

# Generate side to move key
side_to_move_key = secrets.randbits(64)

print(f"pub const BLACK_TO_MOVE: u64 = 0x{side_to_move_key:016X};")

# Generate piece-square keys
piece_square_keys = [[[[secrets.randbits(64) for _ in range(RANKS)] for _ in range(FILES)] for _ in range(PIECE_TYPES)] for _ in range(PLAYERS)]

print("pub const PIECE_SQUARE_KEYS: [[[[u64; 8]; 8]; 6]; 2] = [")
for player_keys in piece_square_keys:
    print("    [")
    for piece_keys in player_keys:
        print("        [")
        for file_keys in piece_keys:
            print("            [", end="")
            print(", ".join(f"0x{key:016X}" for key in file_keys), end="],\n")
        print("        ],")
    print("    ],")
print("];")

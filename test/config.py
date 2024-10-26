import os

TEST_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(TEST_DIR)
BRITTEN_PATH = os.getenv('BRITTEN_PATH', os.path.join(os.path.dirname(os.path.dirname(__file__)), "target/x86_64-unknown-linux-gnu/release/britten"))
# SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
# SPDX-License-Identifier: GPL-3.0-or-later

import os
import time
from pathlib import Path

def wait_for_file_removed(file):
    while os.path.exists(file):
        time.sleep(1)
  
if __name__ == '__main__':
    import sys
    if len(sys.argv) < 2:
        sys.exit("ERROR: missing filename argument")
    Path(sys.argv[1]).touch()
    wait_for_file_removed(sys.argv[1])

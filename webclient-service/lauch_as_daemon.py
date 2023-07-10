# SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
# SPDX-License-Identifier: GPL-3.0-or-later

import os, time, sys, subprocess

sys.argv.pop(0)
print(sys.argv)
subprocess.Popen(sys.argv, close_fds=True)

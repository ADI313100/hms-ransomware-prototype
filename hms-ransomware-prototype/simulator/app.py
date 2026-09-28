
from pathlib import Path
import time
p=Path("test_data"); p.mkdir(exist_ok=True)
print("Simulator running...")
for i in range(10):
    (p/f"file_{i}.txt").write_text("simulated ransomware activity")
    print("Created", i)
    time.sleep(1)

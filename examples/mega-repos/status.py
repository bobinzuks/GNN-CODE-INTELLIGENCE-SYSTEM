#!/usr/bin/env python3
"""
Quick status check for repository generation
"""

import os
import subprocess
from pathlib import Path
from datetime import datetime

def main():
    base_path = Path(__file__).parent

    # Count repos
    repos = sorted([d for d in base_path.iterdir() if d.is_dir() and d.name.startswith('repo-')])
    total = len(repos)

    # Get disk usage
    result = subprocess.run(['du', '-sh', str(base_path)], capture_output=True, text=True)
    disk_usage = result.stdout.split()[0] if result.returncode == 0 else "Unknown"

    # Calculate progress
    target = 1000
    progress = (total / target) * 100

    # Estimate completion
    if total > 0:
        # Assume ~3 minutes per repo on average
        remaining = target - total
        est_minutes = remaining * 3
        est_hours = est_minutes / 60

    print("=" * 70)
    print("MEGA REPOSITORY FACTORY - STATUS")
    print("=" * 70)
    print()
    print(f"Generated:      {total:4d} / {target} repositories")
    print(f"Progress:       {progress:5.1f}%")
    print(f"Disk Usage:     {disk_usage}")
    print()

    if total < target:
        print(f"Remaining:      {target - total:4d} repositories")
        print(f"Est. Time:      {est_hours:.1f} hours")
        print()

    # Show latest repos
    if repos:
        print("Latest 5 Repositories:")
        print("-" * 70)
        for repo in sorted(repos, reverse=True)[:5]:
            # Get commit count
            try:
                os.chdir(repo)
                result = subprocess.run(
                    ['git', 'rev-list', '--count', 'HEAD'],
                    capture_output=True,
                    text=True
                )
                commits = result.stdout.strip() if result.returncode == 0 else "?"

                # Get file count
                files = len([f for f in repo.rglob('*') if f.is_file() and '.git' not in str(f)])

                print(f"  {repo.name:45s} {commits:>4s} commits  {files:>4d} files")
            except:
                print(f"  {repo.name:45s} Error reading stats")

    print("=" * 70)
    print()

    # Check if generation is running
    result = subprocess.run(['pgrep', '-f', 'generate_mega_repos.py'], capture_output=True)
    if result.returncode == 0:
        print("Status: ✓ Generator is currently RUNNING")
    else:
        print("Status: ✗ Generator is NOT running")

        if total < target:
            print()
            print("To resume generation, run:")
            print("  python3 generate_mega_repos.py")
            print()
            print("Or use parallel generation (faster):")
            print("  python3 parallel_generator.py")

    print()


if __name__ == '__main__':
    main()

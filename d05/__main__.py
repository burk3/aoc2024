from collections import defaultdict
from typing import List, Dict, Set

def parse_input(filename: str) -> tuple[Dict[int, Set[int]], List[List[int]]]:
    rules = defaultdict[int, set[int]](set)
    updates = list[list[int]]()
    
    with open(filename) as f:
        # Parse rules
        while True:
            line = f.readline().strip()
            if not line:
                break
            before, after = map(int, line.split('|'))
            rules[before].add(after)
            
        # Parse updates
        for line in f:
            if line.strip():
                updates.append([int(x) for x in line.strip().split(',')])
    
    return rules, updates

def is_valid_update(rules: Dict[int, Set[int]], update: List[int]) -> bool:
    # Create position mapping for this update
    positions = {page: i for i, page in enumerate(update)}
    
    # Check each rule that applies to pages in this update
    for page in update:
        if page in rules:
            for must_come_after in rules[page]:
                if must_come_after in positions:
                    # If rule applies, check positions
                    if positions[must_come_after] < positions[page]:
                        return False
    
    return True

def solve(filename: str) -> int:
    rules, updates = parse_input(filename)
    middle_sum = 0
    
    for update in updates:
        if is_valid_update(rules, update):
            # Find middle number
            middle_index = len(update) // 2
            middle_sum += update[middle_index]
    
    return middle_sum

# Example usage
result = solve("d05/src/input.txt")
print(f"Sum of middle numbers in valid updates: {result}")
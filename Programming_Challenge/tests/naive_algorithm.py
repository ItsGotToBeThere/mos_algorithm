import sys
from collections import defaultdict

def main():
    input_data = sys.stdin.read().split()
    idx = 0

    N = int(input_data[idx]); idx += 1
    arr = [int(input_data[idx + i]) for i in range(N)]; idx += N

    Q = int(input_data[idx]); idx += 1
    K = int(input_data[idx]); idx += 1

    out = []

    for _ in range(Q):
        query_type = input_data[idx]; idx += 1

        if query_type == 'F':
            target = int(input_data[idx]); idx += 1
            l = int(input_data[idx]);     idx += 1
            r = int(input_data[idx]);     idx += 1

            count = 0
            for i in range(l, r + 1):
                if arr[i] == target:
                    count += 1
            out.append(str(count))

        else:  # 'U'
            l = int(input_data[idx]); idx += 1
            r = int(input_data[idx]); idx += 1

            freq = defaultdict(int)
            for i in range(l, r + 1):
                freq[arr[i]] += 1

            unique_count = sum(1 for v in freq.values() if v >= K)
            out.append(str(unique_count))

    print('\n'.join(out))

if __name__ == "__main__":
    main()
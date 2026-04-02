import math

class Query:
    left: int
    right: int
    idx: int

    def __init__(self, left, right, idx):
        self.left = left
        self.right = right
        self.idx = idx


"""
An Implementation of Mo's Algorithm in Python
Inputs will be of the form
x_1 x_2 x_3 ... x_n: int[] // array of length n
num_queries: int // number of queries
left right: int, int // left index and right index you want to query, for q lines
"""

array: list[int] = [int(x) for x in input.split(" ")]
num_queries: int = int(input())

queries = []
for i in range(num_queries):
    left, right = [int(x) for x in input.split(" ")]
    queries.push(Query(left,right,i))



"""
We will be computing Mo's Algorithm where our queries will be the sum of elements in the array
from left to right (inclusive)
"""
def mos_algorithm(array: list[int], queries):
    answers = [0 for _ in len(queries)]
    sorted_queries = sorted(queries, key=lambda x: (x.left,x.right))

    current_left = 0
    current_right = -1
    
    cursum = 0
    for query in sorted_queries:
        while current_left > query.left:
            current_left-=1
            cursum+=array[current_left]
        
        while current_right < query.right:
            current_right+=1
            cursum+=array[current_right]

        while current_left < query.left:
            cursum-=array[current_left]
            current_left+=1
        
        while current_right > query.right:
            cursum-=array[current_right]
            current_right-=1
        
        answers[query.idx] = cursum
    
    for answer in answers:
        print(answer)







"""
Square root decomposition just breaks the array of length n into chuncks of length sqrt{n} so that when we query
over a range, we can add entire chunks at a time if they fall entirely in the range
"""
def sqrt_decomposition(a: list[int]) -> list[int]:
    sqrt_n: int = math.floor(math.sqrt(len(a)))

    b = [0 for _ in range(sqrt_n)]
    for i in range(len(array)):
        b[i/sqrt_n]+=a[i]

    return b


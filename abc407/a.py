a,b=map(int,input().split())
c=a//b
print(c if a-c*b<=b//2 else c+1)
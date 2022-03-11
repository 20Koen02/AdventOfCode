def main():
    limits = [248345, 746315]
    ans = 0

    for x in range(limits[0], limits[1] + 1):
        if checkNeverDecrease(x) and checkDouble(x):
            ans += 1
    print(ans)
    
def checkNeverDecrease(num):
    num = [int(x) for x in str(num)] 
    return num == sorted(num)

def checkDouble(num):
    return any(str(num).count(x) == 2 for x in str(num))

if __name__ == "__main__":
    main()
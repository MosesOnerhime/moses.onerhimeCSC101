# inroduction
print("Welcome to the first semester 100 level computer science GPA calculator:)")
print("")
print("Here, you will be asked to input your score for each of the the 10 courses.")
print("Lets proceed!!")
print("")

# Get the information from the user and change from string to integer
print("Course name        Score")
gst101 = int(input("GST 101             "))
gst108 = int(input("GST 108             "))
csc101 = int(input("CSC 101             "))
chm101 = int(input("CHM 101             "))
chm107 = int(input("CHM 107             "))
phy101 = int(input("PHY 101             "))
phy107 = int(input("PHY 107             "))
mth101 = int(input("MTH 101             "))
mth103 = int(input("MTH 103             "))
gst103 = int(input("GST 103             "))

# If statements

# for gst1o1
if 69.5 <= gst101 <= 100:
    gst1o1s = 5 * 2
elif 59.5 <= gst101 <= 69.4:
    gst1o1s = 4 * 2
elif 49.5 <= gst101 <= 59.4:
    gst1o1s = 3 * 2
elif 44.5 <= gst101 <= 49.4:
    gst1o1s = 2 * 2
elif 0 <= gst101 <= 44.4:
    gst1o1s = 0 * 2

# for gst1o8
if 69.5 <= gst108 <= 100:
    gst1o8s = 5 * 2
elif 59.5 <= gst108 <= 69.4:
    gst1o8s = 4 * 2
elif 49.5 <= gst108 <= 59.4:
    gst1o8s = 3 * 2
elif 44.5 <= gst108 <= 49.4:
    gst1o8s = 2 * 2
elif 0 <= gst108 <= 44.4:
    gst1o8s = 0 * 2

# for csc1o1
if 69.5 <= csc101 <= 100:
    csc1o1s = 5 * 3
elif 59.5 <= csc101 <= 69.4:
    csc1o1s = 4 * 3
elif 49.5 <= csc101 <= 59.4:
    csc1o1s = 3 * 3
elif 44.5 <= csc101 <= 49.4:
    csc1o1s = 2 * 3
elif 0 <= csc101 <= 44.4:
    csc1o1s = 0 * 3

# for chm1o1
if 69.5 <= chm101 <= 100:
    chm1o1s = 5 * 3
elif 59.5 <= chm101 <= 69.4:
    chm1o1s = 4 * 3
elif 49.5 <= chm101 <= 59.4:
    chm1o1s = 3 * 3
elif 44.5 <= chm101 <= 49.4:
    chm1o1s = 2 * 3
elif 0 <= chm101 <= 44.4:
    chm1o1s = 0 * 3

# for chm1o7
if 69.5 <= chm107 <= 100:
    chm1o7s = 5 * 1
elif 59.5 <= chm107 <= 69.4:
    chm1o7s = 4 * 1
elif 49.5 <= chm107 <= 59.4:
    chm1o7s = 3 * 1
elif 44.5 <= chm107 <= 49.4:
    chm1o7s = 2 * 1
elif 0 <= chm107 <= 44.4:
    chm1o7s = 0 * 1

# for phy1o1
if 69.5 <= phy107 <= 100:
    phy1o1s = 5 * 3
elif 59.5 <= phy107 <= 69.4:
    phy1o1s = 4 * 3
elif 49.5 <= phy107 <= 59.4:
    phy1o1s = 3 * 3
elif 44.5 <= phy107 <= 49.4:
    phy1o1s = 2 * 3
elif 0 <= phy107 <= 44.4:
    phy1o1s = 0 * 3

# for phy1o7
if 69.5 <= phy107 <= 100:
    phy1o7s = 5 * 1
elif 59.5 <= phy107 <= 69.4:
    phy1o7s = 4 * 1
elif 49.5 <= phy107 <= 59.4:
    phy1o7s = 3 * 1
elif 44.5 <= phy107 <= 49.4:
    phy1o7s = 2 * 1
elif 0 <= phy107 <= 44.4:
    phy1o7s = 0 * 1

# for mth1o1
if 69.5 <= mth101 <= 100:
    mth1o1s = 5 * 3
elif 59.5 <= mth101 <= 69.4:
    mth1o1s = 4 * 3
elif 49.5 <= mth101 <= 59.4:
    mth1o1s = 3 * 3
elif 44.5 <= mth101 <= 49.4:
    mth1o1s = 2 * 3
elif 0 <= mth101 <= 44.4:
    mth1o1s = 0 * 3

# for mth1o3
if 69.5 <= mth103 <= 100:
    mth1o3s = 5 * 3
elif 59.5 <= mth103 <= 69.4:
    mth1o3s = 4 * 3
elif 49.5 <= mth103 <= 59.4:
    mth1o3s = 3 * 3
elif 44.5 <= mth103 <= 49.4:
    mth1o3s = 2 * 3
elif 0 <= mth103 <= 44.4:
    mth1o3s = 0 * 3

# for gst1o3
if 69.5 <= gst103 <= 100:
    gst1o3s = 5 * 2
elif 59.5 <= gst103 <= 69.4:
    gst1o3s = 4 * 2
elif 49.5 <= gst103 <= 59.4:
    gst1o3s = 3 * 2
elif 44.5 <= gst103 <= 49.4:
    gst1o3s = 2 * 2
elif 0 <= gst103 <= 44.4:
    gst1o3s = 0 * 2

GPA = ((gst1o1s + gst1o3s + gst1o8s + mth1o1s + mth1o3s + chm1o1s + chm1o7s + phy1o1s + phy1o7s + csc1o1s) / 23)

if 4.5 <= GPA <= 5:
    grade = "Excellent, you got a first class!!!"
elif 3.5 <= GPA < 4.5:
    grade = "Weldone, you got a second class upper!!"
elif 2.4 <= GPA < 3.5:
    grade = "Good but not good enough. You got a second class lower:)"
elif 1.5 <= GPA < 2.4:
    grade = "You can do better. yo got a third class."
elif GPA < 1.5:
    grade = "Sorry, but you failed this semester, work harder next time"

GPA = str(GPA)

print("Your GPA for the first semester of 100 level is " + GPA)
print(grade)


def exitt():
    str(input("Goodluck in you next semester!! Click on the 'Enter' key to exit "))
    exit()


exitt()

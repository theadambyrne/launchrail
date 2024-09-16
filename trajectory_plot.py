import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv('./outputs/position.csv')
x, y, z = df['X'], df['Y'], df['Z']

fig = plt.figure()
ax = plt.axes(projection='3d')
ax.plot3D(x, y, z, 'royalblue')

ax.set_xlabel('X')
ax.set_ylabel('Y')
ax.set_zlabel('Z')

plt.title('3D Trajectory Plot')
ax.legend(['Trajectory'])
plt.show()

fig.savefig('./outputs/trajectory.png')

#include <iostream>
using namespace std;

int main() {
	string ss;
	int x = 50, cnt = 0;
	while (getline(cin, ss)) {
		int sign = ss[0] == 'L' ? -1 : 1;
		x += sign * (stoi(ss.substr(1, ss.size()), NULL, 10) % 100);
		if (x < 0) x += 100;
		x %= 100;
		cnt += x == 0;
	}
	cout << cnt << endl;
	return 0;
}

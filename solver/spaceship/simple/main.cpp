#include <iostream>
#include <fstream>
#include <cstdio>
#include <cmath>
#include <vector>
#include <string>
#include <set>
#include <map>
#include <stack>
#include <queue>
#include <deque>
#include <bitset>
#include <algorithm>
#include <complex>
#include <array>
#include <functional>
using namespace std;

#define REP(i,n) for(int i=0; i<n; ++i)
#define FOR(i,a,b) for(int i=a; i<=b; ++i)
#define FORR(i,a,b) for (int i=a; i>=b; --i)
#define ALL(c) (c).begin(), (c).end()

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef vector<double> VD;
typedef vector<VI> VVI;
typedef vector<VL> VVL;
typedef vector<VD> VVD;
typedef pair<int,int> P;
typedef pair<ll,ll> PL;

template<typename T> void chmin(T &a, T b) { if (a > b) a = b; }
template<typename T> void chmax(T &a, T b) { if (a < b) a = b; }

int in() { int x; scanf("%d", &x); return x; }
ll lin() { ll x; scanf("%lld", &x); return x; }

int command(P a) {
    int x = a.first, y = a.second;
    if (y == -1 && x == -1) return 1;
    if (y == -1 && x == 0) return 2;
    if (y == -1 && x == 1) return 3;
    if (y == 0 && x == -1) return 4;
    if (y == 0 && x == 0) return 5;
    if (y == 0 && x == 1) return 6;
    if (y == 1 && x == -1) return 7;
    if (y == 1 && x == 0) return 8;
    if (y == 1 && x == 1) return 9;
    assert(false);
}

bool can_stop(ll vx, ll dx) {
    if (vx <= 0) return true;
    return vx * (vx - 1) / 2 <= dx;
}

VI move_1d(ll dx) {
    VI ans;
    if (dx == 0) return ans;
    if (dx < 0) {
        ans = move_1d(-dx);
        for (int &a : ans) {
            a = -a;
        }
        return ans;
    }
    ll x = 0, vx = 0;
    while (x < dx) {
        for (ll a = 1; a >= -1; a--) {
            if (x + vx + a <= dx && can_stop(vx + a, dx - x - vx - a)) {
                ans.push_back(a);
                vx += a;
                x += vx;
                break;
            }
        }
    }
    if (vx == 1) {
        ans.push_back(-1);
    }
    return ans;
}

// 停止状態から(dx, dy)移動して停止
vector<P> move(ll dx, ll dy) {
    vector<P> ans;
    VI ax = move_1d(dx), ay = move_1d(dy);
    int nx = ax.size(), ny = ay.size();
    REP(i,max(nx,ny)) {
        int x = i < nx ? ax[i] : 0;
        int y = i < ny ? ay[i] : 0;
        ans.push_back(P(x, y));
    }
    // cout << "move(" << dx << ", " << dy << "):" << endl;
    // for (auto &a : ans) {
    //     cout << a.first << " " << a.second << endl;
    // }
    return ans;
}

int main(void) {
    VL xs, ys;
    ll x, y;
    while (cin >> x >> y) {
        xs.push_back(x);
        ys.push_back(y);
    }
    int n = xs.size();
    VI ord(n), visited(n);
    x = 0, y = 0;
    REP(i,n) {
        ll idx = -1;
        ll min_dist = 1LL << 60;
        REP(j,n) {
            if (visited[j]) continue;
            ll dx = xs[j] - x, dy = ys[j] - y;
            ll dist = abs(dx) + abs(dy);
            if (dist < min_dist) {
                min_dist = dist;
                idx = j;
            }
        }
        visited[idx] = 1;
        ord[i] = idx;
        x = xs[idx], y = ys[idx];
    }
    VL xs2(xs), ys2(ys);
    REP(i,n) {
        xs[i] = xs2[ord[i]];
        ys[i] = ys2[ord[i]];
    }
    vector<P> ans;
    x = 0, y = 0;
    ll sum = 0;
    REP(i,n) {
        ll nx = xs[i], ny = ys[i];
        ll dx = nx - x, dy = ny - y;
        auto cs = move(dx, dy);
        for (auto &a : cs) {
            ans.push_back(a);
        }
        sum += abs(dx) + abs(dy);
        x = nx, y = ny;
    }
    cerr << "commands:" << endl;
    for (auto &a : ans) {
        cout << command(a);
    }
    cout << endl;
    cerr << "total distance: " << sum << endl;
    return 0;
}

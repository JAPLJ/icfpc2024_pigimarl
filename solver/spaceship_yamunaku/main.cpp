#include <iostream>
#include <fstream>
#include <vector>
#include <utility>
#include <algorithm>
#include <set>
#include <map>

using namespace std;

unsigned int xor128() {
    static unsigned int x = 123456789, y = 362436069, z = 521288629, w = 88675123;
    unsigned int t;
    t = (x ^ (x << 11));
    x = y;
    y = z;
    z = w;
    return (w = (w ^ (w >> 19)) ^ (t ^ (t >> 8)));
}

int nextInt(int n) { // [0, n)
    return xor128() % n;
}

int nextInt(int l, int r) { // [l, r]
    return xor128() % (r - l + 1) + l;
}

template<typename T>
void shuffle(vector<T> &v) {
    for (int i = v.size(); i >= 2; i--) {
        swap(v[i - 1], v[nextInt(i)]);
    }
}

long long calc_trapezoid_area(int l, int r) {
    if (l > r) swap(l, r);
    int d = r - l + 1;
    return (long long)(l + r) * d / 2;
}

pair<int, int> distance_range(int vs, int vt, int t) {
    if (abs(vt - vs) > t) return {0, -1};
    int min_v = min(vs, vt) - (t - abs(vt - vs)) / 2;
    int max_v = max(vs, vt) + (t - abs(vt - vs)) / 2;
    long long min_d = calc_trapezoid_area(vs, min_v) - vs + calc_trapezoid_area(min_v, vt) - ((t - abs(vt - vs)) & 1 ? 0 : min_v);
    long long max_d = calc_trapezoid_area(vs, max_v) - vs + calc_trapezoid_area(max_v, vt) - ((t - abs(vt - vs)) & 1 ? 0 : max_v);
    return {max(min_d, (long long)-1e9), min(max_d, (long long)1e9)};
}

struct status {
    pair<int, int> p;
    pair<int, int> v;
};

struct seach_result {
    int t;
    vector<int> vxt_range;
    vector<int> vyt_range;
};

// const int search_turn_max = 1000;
// seach_result search(pair<int, int> ps, pair<int, int> pt, pair<int, int> vs) {
//     int dx = pt.first - ps.first;
//     int dy = pt.second - ps.second;
//     int vxs = vs.first;
//     int vys = vs.second;
//     int l = 1, r = search_turn_max;
//     while (r - l > 1) {
//         int mid = (l + r) / 2;
//         if (distance_range(vxs, vxs + mid, mid).second < dx || distance_range(vys, vys + mid, mid).second < dy) {
//             l = mid;
//         } else if (dx < distance_range(vxs, vxs - mid, mid).first || dy < distance_range(vys, vys - mid, mid).first) {
//             r = mid;
//         } else {
//             break;
//         }
//     }
//     // cerr << l << " " << r << endl;
//     for (int mid = l + 1; mid < r; mid++) {
//         vector<int> vxt_range, vyt_range;
//         for (int vxt = vxs - mid; vxt <= vxs + mid; vxt++) {
//             auto p = distance_range(vxs, vxt, mid);
//             // cerr << dx << " " << vxs << " " << vxt << " " << l << ": [" << p.first << ", " << p.second << "]" << endl;
//             if (p.first <= dx && dx <= p.second) {
//                 vxt_range.push_back(vxt);
//             }
//         }
//         for (int vyt = vys - mid; vyt <= vys + mid; vyt++) {
//             auto p = distance_range(vys, vyt, mid);
//             if (p.first <= dy && dy <= p.second) {
//                 vyt_range.push_back(vyt);
//             }
//         }
//         if (vxt_range.size() > 0 && vyt_range.size() > 0) {
//             return {l, vxt_range, vyt_range};
//         }
//     }

//     return {(int) 1e8, {}, {}};
// }

const int wide_search_turn_max = 10000;
vector<seach_result> wide_search(pair<int, int> ps, pair<int, int> pt, pair<int, int> vs) {
    int dx = pt.first - ps.first;
    int dy = pt.second - ps.second;
    int vxs = vs.first;
    int vys = vs.second;
    // cerr << dx << " " << dy << endl;
    vector<seach_result> ans;
    for (int l = 1; l < wide_search_turn_max; l++) {
        vector<int> vxt_range, vyt_range;
        for (int vxt = vxs - l; vxt <= vxs + l; vxt++) {
            auto p = distance_range(vxs, vxt, l);
            // cerr << dx << " " << vxs << " " << vxt << " " << l << ": [" << p.first << ", " << p.second << "]" << endl;
            if (p.first <= dx && dx <= p.second) {
                vxt_range.push_back(vxt);
            }
        }
        for (int vyt = vys - l; vyt <= vys + l; vyt++) {
            auto p = distance_range(vys, vyt, l);
            if (p.first <= dy && dy <= p.second) {
                vyt_range.push_back(vyt);
            }
        }
        if (vxt_range.size() > 0 && vyt_range.size() > 0) {
            ans.push_back({l, vxt_range, vyt_range});
            if (ans.size() >= 5) return ans;
        }
    }
    if (ans.size() > 0) return ans;
    return {(int) 1e9, {}, {}};
}

int search_time(pair<int, int> ps, pair<int, int> pt, pair<int, int> vs, pair<int, int> vt) {
    int dx = pt.first - ps.first;
    int dy = pt.second - ps.second;
    int vxs = vs.first;
    int vys = vs.second;
    int vxt = vt.first;
    int vyt = vt.second;
    for (int l = 1; l < wide_search_turn_max; l++) {
        vector<int> vxt_range, vyt_range;
        bool ok = true;
        auto px = distance_range(vxs, vxt, l);
        if (px.first <= dx && dx <= px.second) ok = false;
        auto py = distance_range(vys, vyt, l);
        if (py.first <= dy && dy <= py.second) ok = false;
        if (ok) return l;
    }
    return (int) 1e9;
}

vector<int> restore(int d, int vs, int vt, int t) {
    vector<int> ans(t + 1);
    int rem = d + vs;
    for (int i = 0; i <= t; i++) {
        ans[i] = max(vs - i, vt - (t - i));
        rem -= ans[i];
    }
    // cerr << "! " << rem << endl;
    int pre_rem = -1;
    while (rem > 0) {
        // cerr << "? " << rem << endl;
        for (int i = 1; i < t; i++) {
            if (ans[i] + 1 < ans[i - 1] - 1 || ans[i - 1] + 1 < ans[i] + 1) continue;
            if (ans[i] + 1 < ans[i + 1] - 1 || ans[i + 1] + 1 < ans[i] + 1) continue;
            ans[i]++;
            rem--;
            if (rem == 0) break;
        }
        // if (pre_rem == rem) {
        //     cerr << d << " " << vs << " " << vt << " " << t << endl;
        //     for (int i = 0; i <= t; i++) {
        //         cerr << ans[i] << " ";
        //     }
        //     cerr << endl;
        //     exit(0);
        // }
        pre_rem = rem;
    }
    return ans;
}

vector<pair<int, int>> restore(pair<int, int> ps, pair<int, int> pt, pair<int, int> vs, pair<int, int> vt, int t) {
    pair<int, int> d = {pt.first - ps.first, pt.second - ps.second};
    vector<int> vx = restore(d.first, vs.first, vt.first, t);
    vector<int> vy = restore(d.second, vs.second, vt.second, t);
    vector<pair<int, int>> ans;
    for (int i = 1; i <= t; i++) {
        ans.emplace_back(vx[i], vy[i]);
    }
    return ans;
}

void validate(vector<pair<int,int>> vertices, string s) {
    int idx = 0;
    pair<int, int> p = {0, 0};
    pair<int, int> v = {0, 0};
    // cerr << s << endl;
    for (auto c : s) {
        int command = c - '0';
        v = {v.first + (command - 1) % 3 - 1, v.second + (command - 1) / 3 - 1};
        p = {p.first + v.first, p.second + v.second};
        // cerr << "pos: (" << p.first << ", " << p.second << "), v:(" << v.first << ", " << v.second << ")" << endl;
        if (idx < vertices.size()) {
            if (p == vertices[idx]) {
                idx++;
            }
        }
    }
    // cerr << idx << " " << vertices.size() << endl;
    assert(idx == vertices.size());
}

int main() {
    // auto p0 = distance_range(3, 4, 5);
    // cerr << p0.first << " " << p0.second << endl;
    // auto p1 = distance_range(3, 4, 6);
    // cerr << p1.first << " " << p1.second << endl;
    // auto p2 = distance_range(3, 4, 7);
    // cerr << p2.first << " " << p2.second << endl;
    // auto p3 = distance_range(4, 3, 7);
    // cerr << p3.first << " " << p3.second << endl;

    vector<pair<int, int>> vertices;
    int x, y;
    while (cin >> x >> y) {
        vertices.emplace_back(x, y);
    }
    sort(vertices.begin(), vertices.end());
    vertices.erase(unique(vertices.begin(), vertices.end()), vertices.end());
    int n = vertices.size();

    set<tuple<int, int, int>> st;
    vector<pair<pair<int, int>, pair<int, int>>> pre_edges;
    map<pair<int, int>, int> zaatsu;
    vector<int> edge_cnt(n);
    for (int i = 0; i < n; i++) {
        map<pair<int,int>, int> mp;
        for (int j = 0; j < n; j++) {
            if (j == i) continue;
            int dx = vertices[j].first - vertices[i].first;
            int dy = vertices[j].second - vertices[i].second;
            mp[{dx, dy}] = j;
            for (int ax = -1; ax <= 1; ax++) {
                for (int ay = -1; ay <= 1; ay++) {
                    int nx = - dx - ax, ny = - dy - ay;
                    auto itr = mp.find({nx, ny});
                    if (itr != mp.end()) {
                        pre_edges.push_back({{itr->second, i}, {i, j}});
                        pre_edges.push_back({{j, i}, {i, itr->second}});
                        zaatsu[{itr->second, i}] = zaatsu[{i, j}] = 0;
                        zaatsu[{j, i}] = zaatsu[{i, itr->second}] = 0;
                        edge_cnt[i]++;
                        st.insert({itr->second, i, j});
                        st.insert({j, i, itr->second});
                    }
                }
            }
        }
    }

    // sort(edge_cnt.begin(), edge_cnt.end());
    for (int i = 0; i < n; i++) {
        cerr << edge_cnt[i] << " " << vertices[i].first << " " << vertices[i].second << endl;
    }
    cerr << endl;

    int vec_num = 0;
    for (auto &p : zaatsu) {
        p.second = vec_num++;
    }
    
    vector<vector<int>> edges(vec_num);
    for (auto pre_edge : pre_edges) {
        edges[zaatsu[pre_edge.first]].push_back(zaatsu[pre_edge.second]);
        edges[zaatsu[pre_edge.second]].push_back(zaatsu[pre_edge.first]);
    }

    vector<int> perm(n);
    for (int i = 0; i < n; i++) perm[i] = i;
    shuffle(perm);
    int best_score = 0;
    for (int i = 0; i < n - 2; i++) {
        if (st.find({perm[i], perm[i + 1], perm[i + 2]}) != st.end()) {
            best_score++;
        }
    }

    cerr << best_score << endl;
    const int trial = 3000000;
    for (int t = 0; t < trial; t++) {
        int op = nextInt(3);
        if (op == 0) {
            int x = nextInt(n);
            int y = nextInt(n);
            if (x == y) continue;
            int d = 0;
            set<int> indices;
            for (int i = -2; i <= 0; i++) {
                if (0 <= x + i && x + i + 2 < n) indices.insert(x + i);
                if (0 <= y + i && y + i + 2 < n) indices.insert(y + i);
            }
            for (auto l : indices) {
                if (st.find({perm[l], perm[l + 1], perm[l + 2]}) != st.end()) d--;
            }
            reverse(perm.begin() + x, perm.begin() + y + 1);
            for (auto l : indices) {
                if (st.find({perm[l], perm[l + 1], perm[l + 2]}) != st.end()) d++;
            }
            if (d >= 0) {
                best_score += d;
                if (d > 0) {
                    cerr << x << " " << y << " " << d << " " << best_score << endl;
                }
            } else {
                reverse(perm.begin() + x, perm.begin() + y + 1);
            }
        } else if (op == 1) {
            int x = nextInt(n);
            int y = nextInt(n);
            if (x == y) continue;
            int d = 0;
            set<int> indices;
            for (int i = -2; i <= 0; i++) {
                if (0 <= x + i && x + i + 2 < n) indices.insert(x + i);
                if (0 <= y + i && y + i + 2 < n) indices.insert(y + i);
            }
            for (auto l : indices) {
                if (st.find({perm[l], perm[l + 1], perm[l + 2]}) != st.end()) d--;
            }
            swap(perm[x], perm[y]);
            for (auto l : indices) {
                if (st.find({perm[l], perm[l + 1], perm[l + 2]}) != st.end()) d++;
            }
            if (d >= 0) {
                best_score += d;
                if (d > 0) {
                    cerr << x << " " << y << " " << d << " " << best_score << endl;
                }
            } else {
                swap(perm[x], perm[y]);
            }
        } else {
            auto rotate = [&](int from, int to) {
                int tmp = perm[from];
                if (from < to) {
                    for (int i = from; i < to; i++) {
                        perm[i] = perm[i + 1];
                    }
                } else {
                    for (int i = from; i > to; i--) {
                        perm[i] = perm[i - 1];
                    }
                }
                perm[to] = tmp;
            };
            int x = nextInt(n);
            int y = nextInt(n);
            if (x == y) continue;
            int d = 0;
            set<int> indices;
            // for (int i = -2; i <= 0; i++) {
            //     if (0 <= x + i && x + i + 2 < n) indices.insert(x + i);
            //     if (0 <= y + i && y + i + 2 < n) indices.insert(y + i);
            // }
            for (int i = min(x, y) - 2; i <= max(x, y); i++) indices.insert(i);
            for (auto l : indices) {
                if (st.find({perm[l], perm[l + 1], perm[l + 2]}) != st.end()) d--;
            }
            rotate(x, y);
            for (auto l : indices) {
                if (st.find({perm[l], perm[l + 1], perm[l + 2]}) != st.end()) d++;
            }
            if (d >= 0) {
                best_score += d;
                if (d > 0) {
                    cerr << x << " " << y << " " << d << " " << best_score << endl;
                }
            } else {
                rotate(y, x);
            }
        }
    }
    cerr << best_score << endl;

    vector<int> now_group = {perm[0]};
    vector<vector<int>> groups;
    for (int i = 1; i < n - 1; i++) {
        if (st.find({perm[i - 1], perm[i], perm[i + 1]}) != st.end()) {
            now_group.push_back(perm[i]);
        } else {
            if (now_group.size() == 1) {
                groups.push_back(now_group);
                now_group = {perm[i]};
            } else {
                now_group.push_back(perm[i]);
                groups.push_back(now_group);
                now_group = {perm[i + 1]};
                i++;
            }
        }
    }
    if (now_group.size() == 1) {
        groups.push_back(now_group);
        now_group = {perm.back()};
        groups.push_back(now_group);
    } else {
        now_group.push_back(perm.back());
        groups.push_back(now_group);
    }

    int m = groups.size();
    vector<int> g_perm(m);
    for (int i = 0; i < m; i++) g_perm[i] = i;
    shuffle(g_perm);

    auto calc_g_score = [&]() {
        int g_score = 0;
        pair<int, int> p = {0, 0}, v = {0, 0};
        for (int i = 0; i < m; i++) {
            const auto &group = groups[g_perm[i]];
            if (group.size() > 1) {
                g_score += search_time(p, vertices[group.front()], v, {
                    vertices[group[1]].first - vertices[group[0]].first,
                    vertices[group[1]].second - vertices[group[0]].second
                });
                p = vertices[group.back()];
                v = {
                    vertices[group.back()].first - vertices[group[group.size() - 2]].first,
                    vertices[group.back()].second - vertices[group[group.size() - 2]].second
                };
            } else {
                int g = group[0];
                auto result = wide_search(p, vertices[g], v);
                g_score += result.front().t;
                if (g_score >= 1e9) return (int) 1e9;
                p = vertices[g];
                v = {
                    result.front().vxt_range.front(),
                    result.front().vyt_range.front()
                };
            }
        }
        return (int) g_score;
    };

    int g_best_score = calc_g_score();
    const int g_trial = 10000;
    cerr << g_best_score << endl;
    for (int t = 0; t < g_trial; t++) {
        int op = nextInt(4);
        if (op == 0) {
            int x = nextInt(m);
            int y = nextInt(m);
            if (x == y) continue;
            reverse(g_perm.begin() + x, g_perm.begin() + y + 1);
            int g_score = calc_g_score();
            if (g_score < g_best_score) {
                g_best_score = g_score;
                cerr << g_best_score << endl;
            } else {
                reverse(g_perm.begin() + x, g_perm.begin() + y + 1);
            }
        } else if (op == 1) {
            int x = nextInt(m);
            int y = nextInt(m);
            if (x == y) continue;
            swap(g_perm[x], g_perm[y]);
            int g_score = calc_g_score();
            if (g_score < g_best_score) {
                g_best_score = g_score;
                cerr << g_best_score << endl;
            } else {
                swap(g_perm[x], g_perm[y]);
            }
        } else if (op == 2) {
            auto rotate = [&](int from, int to) {
                int tmp = g_perm[from];
                if (from < to) {
                    for (int i = from; i < to; i++) {
                        g_perm[i] = g_perm[i + 1];
                    }
                } else {
                    for (int i = from; i > to; i--) {
                        g_perm[i] = g_perm[i - 1];
                    }
                }
                g_perm[to] = tmp;
            };
            int x = nextInt(m);
            int y = nextInt(m);
            if (x == y) continue;
            rotate(x, y);
            int g_score = calc_g_score();
            if (g_score < g_best_score) {
                g_best_score = g_score;
                cerr << g_best_score << endl;
            } else {
                rotate(y, x);
            }
        } else {
            int x = nextInt(m);
            reverse(groups[g_perm[x]].begin(), groups[g_perm[x]].end());
            int g_score = calc_g_score();
            if (g_score < g_best_score) {
                g_best_score = g_score;
                cerr << g_best_score << endl;
            } else {
                reverse(groups[g_perm[x]].begin(), groups[g_perm[x]].end());
            }
        }
    }
    cerr << "complete" << endl;

    vector<pair<int, int>> new_vertices;
    for (int i = 0; i < m; i++) {
        for (auto idx : groups[g_perm[i]]) new_vertices.push_back(vertices[idx]);
    }
    set<int> pst;
    for (int i = 0; i < m; i++) {
        for (auto idx : groups[g_perm[i]]) {
            assert (pst.find(idx) == pst.end());
            pst.insert(idx);
        }
    }
    for (int i = 0; i < n; i++) {
        assert (pst.find(i) != pst.end());
    }
    vertices = new_vertices;
    // reverse(vertices.begin(), vertices.end());
    // for (int i = 0; i < n; i++) {}

    // sort(vertices.begin(), vertices.end(), [&](const pair<int, int> &l, pair<int, int> &r) {
    //     return l.second == r.second ? l.first < r.first : l.second < r.second;
    // });

    pair<int, int> p = {0, 0};
    pair<int, int> v = {0, 0};
    vector<pair<int, int>> vs;
    for (int i = 0; i < n; i++) {
        auto results = wide_search(p, vertices[i], v);
        auto new_p = vertices[i];
        pair<int, int> new_v = {results.front().vxt_range.front(), results.front().vyt_range.front()};
        int result_t = results.front().t;
        // if (i < n - 1) { // 1 手先読み
        //     const int v_threshold = 1e9; // ケースに応じて変える
        //     pair<int, pair<int, int>> best = {-1, {-1, -1}};
        //     int best_turn = 1e9;
        //     for (auto result : results) {
        //         sort(result.vxt_range.begin(), result.vxt_range.end());
        //         sort(result.vyt_range.begin(), result.vyt_range.end());
        //         vector<int> vxt_candidates, vyt_candidates;
        //         for (int i = 0; i < 10; i++) {
        //             vxt_candidates.push_back(result.vxt_range[result.vxt_range.size() * i / 10]);
        //             vyt_candidates.push_back(result.vyt_range[result.vyt_range.size() * i / 10]);
        //         }
        //         vxt_candidates.push_back(result.vxt_range.back());
        //         vyt_candidates.push_back(result.vyt_range.back());
        //         vxt_candidates.erase(unique(vxt_candidates.begin(), vxt_candidates.end()), vxt_candidates.end());
        //         vyt_candidates.erase(unique(vyt_candidates.begin(), vyt_candidates.end()), vyt_candidates.end());

        //         for (auto vxt : vxt_candidates) {
        //             if (abs(vxt) > v_threshold) continue;
        //             for (auto vyt : vyt_candidates) {
        //                 if (abs(vyt) > v_threshold) continue;
        //                 auto tmp_result = search(vertices[i], vertices[i + 1], {vxt, vyt});
        //                 int tmp_turn = tmp_result.t + result.t;
        //                 if (tmp_turn < best_turn) {
        //                     best_turn =  tmp_turn;
        //                     best = {result.t, {vxt, vyt}};
        //                 }
        //             }
        //         }
        //     }
        //     result_t = best.first;
        //     new_v = best.second;
        // }
        auto new_vs = restore(p, new_p, v, new_v, result_t);
        cerr << "pos: (" << new_p.first << ", " << new_p.second << "), v:(" << new_v.first << ", " << new_v.second << "), time: " << result_t << endl;

        vs.insert(vs.end(), new_vs.begin(), new_vs.end());
        p = new_p;
        v = new_v;
    }

    string ans = "";
    pair<int, int> now_v = {0, 0};
    for (auto next_v : vs) {
        pair<int, int> a = {
            next_v.first - now_v.first,
            next_v.second - now_v.second
        };
        now_v = next_v;
        int command = (a.second + 1) * 3 + (a.first + 1) + 1;
        ans.push_back('0' + command);
    }

    validate(vertices, ans);

    cout << ans << endl;
}
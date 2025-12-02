#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <map>
#include <algorithm>
#include <chrono>
#include <iomanip>
using namespace std;
using ll = long long;

vector<int> get_divisors(int n) {
    vector<int> divs;
    for (int i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            if (i < n) divs.push_back(i);
            if (i != n / i && n / i < n) divs.push_back(n / i);
        }
    }
    sort(divs.begin(), divs.end());
    return divs;
}

ll pow10[20];

ll make_periodic(ll prefix, int k, int reps) {
    ll result = 0;
    for (int i = 0; i < reps; i++)
        result = result * pow10[k] + prefix;
    return result;
}

ll first_periodic_geq(ll lo, int num_digits, int k) {
    int reps = num_digits / k;
    ll min_prefix = pow10[k - 1], max_prefix = pow10[k] - 1;

    string lo_str = to_string(lo);
    while ((int)lo_str.size() < num_digits) lo_str = "0" + lo_str;
    ll lo_prefix = stoll(lo_str.substr(0, k));

    ll candidate = make_periodic(lo_prefix, k, reps);
    if (candidate >= lo)
        return lo_prefix >= min_prefix ? candidate : make_periodic(min_prefix, k, reps);

    lo_prefix++;
    if (lo_prefix > max_prefix) return -1;
    return make_periodic(max(lo_prefix, min_prefix), k, reps);
}

ll last_periodic_leq(ll hi, int num_digits, int k) {
    int reps = num_digits / k;
    ll min_prefix = pow10[k - 1], max_prefix = pow10[k] - 1;

    string hi_str = to_string(hi);
    while ((int)hi_str.size() < num_digits) hi_str = "0" + hi_str;
    ll hi_prefix = stoll(hi_str.substr(0, k));

    ll candidate = make_periodic(hi_prefix, k, reps);
    if (candidate <= hi) {
        if (hi_prefix >= min_prefix && hi_prefix <= max_prefix) return candidate;
        return hi_prefix > max_prefix ? make_periodic(max_prefix, k, reps) : -1;
    }

    hi_prefix--;
    return hi_prefix < min_prefix ? -1 : make_periodic(hi_prefix, k, reps);
}

pair<ll, ll> count_periodic_in_range(ll lo, ll hi, int num_digits, int k) {
    ll first = first_periodic_geq(lo, num_digits, k);
    if (first < 0 || first > hi) return {0, 0};

    ll last = last_periodic_leq(hi, num_digits, k);
    if (last < 0 || last < lo || first > last) return {0, 0};

    int reps = num_digits / k;
    string fs = to_string(first), ls = to_string(last);
    ll first_prefix = stoll(fs.substr(0, k)), last_prefix = stoll(ls.substr(0, k));
    ll count = last_prefix - first_prefix + 1;

    ll multiplier = (pow10[reps * k] - 1) / (pow10[k] - 1);
    ll prefix_sum = (first_prefix + last_prefix) * count / 2;

    return {count, prefix_sum * multiplier};
}

ll sum_invalid_fixed_digits(ll lo, ll hi, int num_digits) {
    auto divs = get_divisors(num_digits);
    if (divs.empty()) return 0;

    map<int, pair<ll, ll>> periodic;
    for (int k : divs)
        periodic[k] = count_periodic_in_range(lo, hi, num_digits, k);

    map<int, ll> min_cnt, min_sum;
    for (int k : divs) {
        auto [cnt, s] = periodic[k];
        for (int q : divs)
            if (q < k && k % q == 0) {
                cnt -= min_cnt[q];
                s -= min_sum[q];
            }
        min_cnt[k] = cnt;
        min_sum[k] = s;
    }

    ll total = 0;
    for (auto& [k, s] : min_sum) total += s;
    return total;
}

ll solve(ll lo, ll hi) {
    ll total = 0;
    ll current = lo;
    while (current <= hi) {
        int num_digits = to_string(current).size();
        ll range_end = min(hi, pow10[num_digits] - 1);
        total += sum_invalid_fixed_digits(current, range_end, num_digits);
        current = range_end + 1;
    }
    return total;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    pow10[0] = 1;
    for (int i = 1; i < 20; i++) pow10[i] = pow10[i - 1] * 10;

    auto start = chrono::high_resolution_clock::now();

    string input;
    getline(cin, input);
    while (!input.empty() && (input.back() == ',' || input.back() == '\n'))
        input.pop_back();

    ll total = 0;
    stringstream ss(input);
    string part;
    while (getline(ss, part, ',')) {
        size_t dash = part.find('-');
        if (dash != string::npos) {
            ll a = stoll(part.substr(0, dash));
            ll b = stoll(part.substr(dash + 1));
            total += solve(a, b);
        }
    }

    auto end = chrono::high_resolution_clock::now();
    double elapsed = chrono::duration<double, milli>(end - start).count();

    cout << total << "\n";
    cerr << fixed << setprecision(3) << elapsed << " ms\n";
    return 0;
}

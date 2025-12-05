#include <iostream>
#include <type_traits>
#include "input.h"

// ============================================================================
// PART 1: COUNT IDS IN RANGES (Pure TMP)
// ============================================================================

// Check if ID at index I is in range at index R
template<int I, int R>
struct IdInRange {
    static constexpr bool value = (IDS[I] >= RANGES[R].lo) && (IDS[I] <= RANGES[R].hi);
};

// Binary search through ranges to check if ID is in any (O(log n) depth)
template<int I, int Lo, int Hi>
struct IdInAnyRangeImpl {
private:
    static constexpr int Mid = (Lo + Hi) / 2;
public:
    static constexpr bool value = IdInAnyRangeImpl<I, Lo, Mid>::value ||
                                   IdInAnyRangeImpl<I, Mid+1, Hi>::value;
};

template<int I, int R>
struct IdInAnyRangeImpl<I, R, R> {
    static constexpr bool value = IdInRange<I, R>::value;
};

template<int I>
struct IdInAnyRange {
    static constexpr bool value = IdInAnyRangeImpl<I, 0, NUM_RANGES-1>::value;
};

// Binary recursion to count IDs in ranges (O(log n) depth)
template<int Lo, int Hi>
struct CountInRangesImpl {
private:
    static constexpr int Mid = (Lo + Hi) / 2;
public:
    static constexpr int value = CountInRangesImpl<Lo, Mid>::value +
                                  CountInRangesImpl<Mid+1, Hi>::value;
};

template<int I>
struct CountInRangesImpl<I, I> {
    static constexpr int value = IdInAnyRange<I>::value ? 1 : 0;
};

constexpr int part1 = CountInRangesImpl<0, NUM_IDS-1>::value;

// ============================================================================
// PART 2: TOTAL COVERAGE (Pure TMP with merge sort)
// ============================================================================

// Index list type
template<int... Is>
struct IndexList {};

// Prepend an index to a list
template<int I, typename L>
struct PrependIndex;

template<int I, int... Is>
struct PrependIndex<I, IndexList<Is...>> {
    using type = IndexList<I, Is...>;
};

// Take first N elements (with enable_if to disambiguate)
template<int N, typename L, typename = void>
struct TakeIndex;

template<typename L>
struct TakeIndex<0, L, void> {
    using type = IndexList<>;
};

template<int N, int I, int... Is>
struct TakeIndex<N, IndexList<I, Is...>, std::enable_if_t<(N > 0)>> {
    using type = typename PrependIndex<I, typename TakeIndex<N-1, IndexList<Is...>>::type>::type;
};

// Drop first N elements
template<int N, typename L, typename = void>
struct DropIndex;

template<typename L>
struct DropIndex<0, L, void> {
    using type = L;
};

template<int N, int I, int... Is>
struct DropIndex<N, IndexList<I, Is...>, std::enable_if_t<(N > 0)>> {
    using type = typename DropIndex<N-1, IndexList<Is...>>::type;
};

// Merge two sorted index lists (by RANGES[i].lo)
template<typename L1, typename L2>
struct MergeSortedIndex;

template<>
struct MergeSortedIndex<IndexList<>, IndexList<>> {
    using type = IndexList<>;
};

template<int... Is>
struct MergeSortedIndex<IndexList<Is...>, IndexList<>> {
    using type = IndexList<Is...>;
};

template<int... Is>
struct MergeSortedIndex<IndexList<>, IndexList<Is...>> {
    using type = IndexList<Is...>;
};

template<int I, int... Is, int J, int... Js>
struct MergeSortedIndex<IndexList<I, Is...>, IndexList<J, Js...>> {
    using type = std::conditional_t<
        (RANGES[I].lo <= RANGES[J].lo),
        typename PrependIndex<I, typename MergeSortedIndex<IndexList<Is...>, IndexList<J, Js...>>::type>::type,
        typename PrependIndex<J, typename MergeSortedIndex<IndexList<I, Is...>, IndexList<Js...>>::type>::type
    >;
};

// Merge sort on index list
template<typename L>
struct SortIndex;

template<>
struct SortIndex<IndexList<>> {
    using type = IndexList<>;
};

template<int I>
struct SortIndex<IndexList<I>> {
    using type = IndexList<I>;
};

template<int I, int J, int... Is>
struct SortIndex<IndexList<I, J, Is...>> {
private:
    static constexpr int N = 2 + sizeof...(Is);
    static constexpr int Half = N / 2;
    using Input = IndexList<I, J, Is...>;
    using Left = typename TakeIndex<Half, Input>::type;
    using Right = typename DropIndex<Half, Input>::type;
    using SortedLeft = typename SortIndex<Left>::type;
    using SortedRight = typename SortIndex<Right>::type;
public:
    using type = typename MergeSortedIndex<SortedLeft, SortedRight>::type;
};

// Generate IndexList<0, 1, ..., N-1>
template<int N, int... Is>
struct MakeIndexListImpl {
    using type = typename MakeIndexListImpl<N-1, N-1, Is...>::type;
};

template<int... Is>
struct MakeIndexListImpl<0, Is...> {
    using type = IndexList<Is...>;
};

template<int N>
using MakeIndexList = typename MakeIndexListImpl<N>::type;

// Sort the range indices
using SortedIndices = typename SortIndex<MakeIndexList<NUM_RANGES>>::type;

// State for merging: current range bounds and accumulated sum
template<long long Lo, long long Hi, long long Sum>
struct MergeState {};

// Process sorted indices, merging overlapping ranges
template<typename State, typename Indices>
struct ProcessMerge;

// Done: add final range to sum
template<long long Lo, long long Hi, long long Sum>
struct ProcessMerge<MergeState<Lo, Hi, Sum>, IndexList<>> {
    static constexpr long long value = Sum + (Hi - Lo + 1);
};

// Process next range
template<long long Lo, long long Hi, long long Sum, int I, int... Is>
struct ProcessMerge<MergeState<Lo, Hi, Sum>, IndexList<I, Is...>> {
private:
    static constexpr long long new_lo = RANGES[I].lo;
    static constexpr long long new_hi = RANGES[I].hi;
    static constexpr bool overlaps = (new_lo <= Hi + 1);
    static constexpr long long next_lo = overlaps ? Lo : new_lo;
    static constexpr long long next_hi = overlaps ? (new_hi > Hi ? new_hi : Hi) : new_hi;
    static constexpr long long next_sum = overlaps ? Sum : (Sum + (Hi - Lo + 1));
public:
    static constexpr long long value = ProcessMerge<MergeState<next_lo, next_hi, next_sum>, IndexList<Is...>>::value;
};

// Compute coverage from sorted indices
template<typename SortedIndices>
struct ComputeCoverage;

template<int I, int... Is>
struct ComputeCoverage<IndexList<I, Is...>> {
    static constexpr long long value = ProcessMerge<
        MergeState<RANGES[I].lo, RANGES[I].hi, 0>,
        IndexList<Is...>
    >::value;
};

template<>
struct ComputeCoverage<IndexList<>> {
    static constexpr long long value = 0;
};

constexpr long long part2 = ComputeCoverage<SortedIndices>::value;

// ============================================================================
// MAIN
// ============================================================================

int main() {
    static_assert(part1 >= 0, "Part 1 computed at compile time");
    static_assert(part2 >= 0, "Part 2 computed at compile time");

    std::cout << "Part 1: " << part1 << std::endl;
    std::cout << "Part 2: " << part2 << std::endl;
    return 0;
}

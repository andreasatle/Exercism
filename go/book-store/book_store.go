// Package bookstrore - To try and encourage more sales of different books from a
// popular 5 book series, a bookshop has decided to offer discounts on multiple
// book purchases.
//
// One copy of any of the five books costs $8.
//
// If, however, you buy two different books, you get a 5% discount on those two books.
//
// If you buy 3 different books, you get a 10% discount.
//
// If you buy 4 different books, you get a 20% discount.
//
// If you buy all 5, you get a 25% discount.
//
// Note: that if you buy four books, of which 3 are different titles, you get a
// 10% discount on the 3 that form part of a set, but the fourth book still costs $8.
//
// Your mission is to write a piece of code to calculate the price of any conceivable
// shopping basket (containing only books of the same series),
// giving as big a discount as possible.
package bookstore

// local cache for the different baskets.
var cache BookCache

// Different prices in cents.
var prices = [6]int{0, 800, 1520, 2160, 2560, 3000}

// Initialize the cache once
func init() {
	cache = BookCache{}
}

// Cost computed the minimal cost for the book basket.
func Cost(basket []int) int {
	return recCost(getBooks(basket))
}

// recCost computes the minimal cost recursively, using a cache.
func recCost(books Books) int {

	// Descending sort of the books for convenience
	books = sort(books)

	// Used cached result if already computed
	if cost, ok := cache[books]; ok {
		return cost
	}

	// Hm, this is a bit repeated, possibly 5 recursions,
	// on the verge of having to be abstracted.
	// Possibly a second recursive function...
	minCost := 0
	if books[0] > 0 {
		books[0]--
		minCost = prices[1] + recCost(books)
		if books[1] > 0 {
			books[1]--
			cost := prices[2] + recCost(books)
			if cost < minCost {
				minCost = cost
			}
			if books[2] > 0 {
				books[2]--
				cost := prices[3] + recCost(books)
				if cost < minCost {
					minCost = cost
				}
				if books[3] > 0 {
					books[3]--
					cost := prices[4] + recCost(books)
					if cost < minCost {
						minCost = cost
					}
					if books[4] > 0 {
						books[4]--
						cost := prices[5] + recCost(books)
						if cost < minCost {
							minCost = cost
						}
						books[4]++
					}
					books[3]++
				}
				books[2]++
			}
			books[1]++
		}
		books[0]++
	}

	// Cache the result
	cache[books] = minCost

	return minCost
}

func getBooks(basket []int) Books {
	books := Books{}
	for _, book := range basket {
		books[book-1]++
	}
	return books
}

type Books [5]int
type BookCache map[Books]int

func sort(books Books) Books {
	for i := 0; i < 4; i++ {
		for j := 0; j < 4-i; j++ {
			if books[j] < books[j+1] {
				books[j], books[j+1] = books[j+1], books[j]
			}
		}
	}
	return books
}

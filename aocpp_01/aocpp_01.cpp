#include <ios>
#include <concepts>
#include <iostream>
#include <fstream>
#include <ranges>
#include <iomanip>
#include <algorithm>
#include <optional>
#include <string_view>
#include <experimental/filesystem>

namespace lfr {

namespace detail
{
    template<typename T>
        using with_ref = T&;

    template<typename T>
        concept can_reference = requires { typename with_ref<T>; };
    
    // Alias for a type that is conditionally const.
    template<bool Const, typename T>
        using maybe_const_t = std::conditional_t<Const, const T, T>;
} // namespace detail

namespace ranges {

template<std::ranges::input_range V, class S, std::copy_constructible F>
    requires std::ranges::view<V> &&
             std::is_object_v<F> &&
             std::is_object_v<S> &&
             std::is_move_constructible_v<S> &&
             std::regular_invocable<F&, std::remove_cv_t<S>&, std::ranges::range_reference_t<V>> &&
             lfr::detail::can_reference<std::invoke_result_t<F&, std::remove_cv_t<S>&, std::ranges::range_reference_t<V>>>
class scan_view : public std::ranges::view_interface<scan_view<V, S, F>>
{
private:
    template<bool Const>
	using Base = lfr::detail::maybe_const_t<Const, V>;

    template<bool Const>
	struct iter_cat
	{ };

    template<bool Const>
	requires std::ranges::forward_range<Base<Const>>
	struct iter_cat<Const>
	{
	private:
	    static auto
	    S_iter_cat()
	    {
            using Base = scan_view::Base<Const>;
            using Res = std::invoke_result_t<F&, std::remove_cv_t<S>&, std::ranges::range_reference_t<Base>>;
            if constexpr (std::is_lvalue_reference_v<Res>)
            {
                using Cat = typename std::iterator_traits<std::ranges::iterator_t<Base>>::iterator_category;
                if constexpr (std::derived_from<Cat, std::contiguous_iterator_tag>)
                    return std::random_access_iterator_tag{};
                else
                    return Cat{};
            }
            else
                return std::input_iterator_tag{};
        }
	public:
	    using iterator_category = decltype(S_iter_cat());
	};

    template<bool Const>
	struct Sentinel;

    template<bool Const>
	struct Iterator : iter_cat<Const>
	{
	private:
	  using Parent = detail::maybe_const_t<Const, scan_view>;
	  using Base = scan_view::Base<Const>;

	  static auto
	  S_iter_concept()
	  {
	    if constexpr (std::ranges::random_access_range<V>)
	        return std::random_access_iterator_tag{};
	    else if constexpr (std::ranges::bidirectional_range<V>)
	        return std::bidirectional_iterator_tag{};
	    else if constexpr (std::ranges::forward_range<V>)
	        return std::forward_iterator_tag{};
	    else
	        return std::input_iterator_tag{};
	  }

	  using Base_iter = std::ranges::iterator_t<Base>;

	  Base_iter m_current = Base_iter();
	  Parent* m_parent = nullptr;

	public:
	    using iterator_concept = decltype(S_iter_concept());
        // iterator_category defined in scan_view_iter_cat
        using value_type = std::remove_cvref_t<std::invoke_result_t<F&, std::remove_cv_t<S>&, std::ranges::range_reference_t<Base>>>;
        using difference_type = std::ranges::range_difference_t<Base>;

        Iterator() = default;

        constexpr
        Iterator(Parent* parent, Base_iter current)
            : m_current(std::move(current))
            , m_parent(parent)
        { }

        constexpr
        Iterator(Iterator<!Const> i)
            requires Const
                && std::convertible_to<std::ranges::iterator_t<V>, Base_iter>
            : m_current(std::move(i.m_current))
            , m_parent(i.m_parent)
        { }

        constexpr Base_iter
        base() const &
            requires std::copyable<Base_iter>
        { return m_current; }

        constexpr Base_iter
        base() &&
        { return std::move(m_current); }

        constexpr decltype(auto)
        operator*() const
            noexcept(noexcept(std::invoke(m_parent->m_fun, m_parent->m_state, *m_current)))
        { return std::invoke(m_parent->m_fun, m_parent->m_state, *m_current); }

        constexpr Iterator&
        operator++()
        {
            ++m_current;
            return *this;
        }

        constexpr void
        operator++(int)
        { ++m_current; }

        constexpr Iterator
        operator++(int) requires std::ranges::forward_range<Base>
        {
            auto tmp = *this;
            ++*this;
            return tmp;
        }

        constexpr Iterator&
        operator--() requires std::ranges::bidirectional_range<Base>
        {
            --m_current;
            return *this;
        }

        constexpr Iterator
        operator--(int) requires std::ranges::bidirectional_range<Base>
        {
            auto tmp = *this;
            --*this;
            return tmp;
        }

        constexpr Iterator&
        operator+=(difference_type n) requires std::ranges::random_access_range<Base>
        {
            m_current += n;
            return *this;
        }

        constexpr Iterator&
        operator-=(difference_type n) requires std::ranges::random_access_range<Base>
        {
            m_current -= n;
            return *this;
        }

        constexpr decltype(auto)
        operator[](difference_type n) const
            requires std::ranges::random_access_range<Base>
        { return std::invoke(m_parent->m_fun, m_parent->m_state, m_current[n]); }

        friend constexpr bool
        operator==(const Iterator& x, const Iterator& y)
            requires std::equality_comparable<Base_iter>
        { return x.m_current == y.m_current; }

        friend constexpr bool
        operator<(const Iterator& x, const Iterator& y)
            requires std::ranges::random_access_range<Base>
        { return x.m_current < y.m_current; }

        friend constexpr bool
        operator>(const Iterator& x, const Iterator& y)
            requires std::ranges::random_access_range<Base>
        { return y < x; }

        friend constexpr bool
        operator<=(const Iterator& x, const Iterator& y)
            requires std::ranges::random_access_range<Base>
        { return !(y < x); }

        friend constexpr bool
        operator>=(const Iterator& x, const Iterator& y)
            requires std::ranges::random_access_range<Base>
        { return !(x < y); }

        friend constexpr auto
        operator<=>(const Iterator& x, const Iterator& y)
            requires std::ranges::random_access_range<Base>
            && std::three_way_comparable<Base_iter>
        { return x.m_current <=> y.m_current; }

        friend constexpr Iterator
        operator+(Iterator i, difference_type n)
            requires std::ranges::random_access_range<Base>
        { return {i.m_parent, i.m_current + n}; }

        friend constexpr Iterator
        operator+(difference_type n, Iterator i)
            requires std::ranges::random_access_range<Base>
        { return {i.m_parent, i.m_current + n}; }

        friend constexpr Iterator
        operator-(Iterator i, difference_type n)
            requires std::ranges::random_access_range<Base>
        { return {i.m_parent, i.m_current - n}; }

        // _GLIBCXX_RESOLVE_LIB_DEFECTS
        // 3483. transform_view::iterator's difference is overconstrained
        friend constexpr difference_type
        operator-(const Iterator& x, const Iterator& y)
            requires std::sized_sentinel_for<std::ranges::iterator_t<Base>, std::ranges::iterator_t<Base>>
        { return x.m_current - y.m_current; }

        friend constexpr decltype(auto)
        iter_move(const Iterator& i) noexcept(noexcept(*i))
        {
            if constexpr (std::is_lvalue_reference_v<decltype(*i)>)
                return std::move(*i);
            else
                return *i;
        }

        friend constexpr void
        iter_swap(const Iterator& x, const Iterator& y)
            noexcept(noexcept(std::ranges::iter_swap(x.m_current, y.m_current)))
            requires std::indirectly_swappable<Base_iter>
        { return std::ranges::iter_swap(x.m_current, y.m_current); }

        friend Iterator<!Const>;
        template<bool> friend struct Sentinel;
    };

    template<bool Const>
	struct Sentinel
	{
	private:
        using Parent = detail::maybe_const_t<Const, scan_view>;
        using Base = scan_view::Base<Const>;

	    template<bool Const2>
	    constexpr auto
	    distance_from(const Iterator<Const2>& i) const
	    { return m_end - i.m_current; }

        template<bool Const2>
        constexpr bool
        equal(const Iterator<Const2>& i) const
        { return i.m_current == m_end; }

	    std::ranges::sentinel_t<Base> m_end = std::ranges::sentinel_t<Base>();

	public:
	    Sentinel() = default;

        constexpr explicit
        Sentinel(std::ranges::sentinel_t<Base> end)
            : m_end(end)
        { }

        constexpr
        Sentinel(Sentinel<!Const> i)
            requires _Const
            && std::convertible_to<std::ranges::sentinel_t<V>, std::ranges::sentinel_t<Base>>
            : m_end(std::move(i.m_end))
        { }

        constexpr std::ranges::sentinel_t<Base>
        base() const
        { return m_end; }

        template<bool Const2>
            requires std::sentinel_for<std::ranges::sentinel_t<Base>,
                std::ranges::iterator_t<detail::maybe_const_t<Const2, V>>>
        friend constexpr bool
        operator==(const Iterator<Const2>& x, const Sentinel& y)
        { return y.equal(x); }

        template<bool Const2, typename Base2 = detail::maybe_const_t<Const2, V>>
            requires std::sized_sentinel_for<std::ranges::sentinel_t<Base>, std::ranges::iterator_t<Base2>>
        friend constexpr std::ranges::range_difference_t<Base2>
        operator-(const Iterator<Const2>& x, const Sentinel& y)
        { return -y.distance_from(x); }

	    template<bool Const2,
		    typename Base2 = detail::maybe_const_t<Const2, V>>
	        requires std::sized_sentinel_for<std::ranges::sentinel_t<Base>, std::ranges::iterator_t<Base2>>
	    friend constexpr std::ranges::range_difference_t<Base2>
	    operator-(const Sentinel& y, const Iterator<Const2>& x)
	    { return y.distance_from(x); }

	    friend Sentinel<!Const>;
	};

    using state_t = std::remove_cv_t<S>;
    V           m_base {};
    state_t     m_state {};
    F           m_fun {};
public:
    scan_view() = default;
    
    constexpr
    scan_view(V base, S initial_state, F function)
        : m_base(std::move(base))
        , m_state(std::move(initial_state))
        , m_fun(std::move(function))
    {}
    
    constexpr V
    base() const & requires std::copy_constructible<V>
    { return m_base; }

    constexpr V
    base() && 
    { return std::move(m_base); }

    constexpr Iterator<false>
    begin()
    { return Iterator<false>{this, std::ranges::begin(m_base)}; }

    constexpr Iterator<true>
    begin() const
    requires std::ranges::range<const V>
    && std::regular_invocable<const F&, std::remove_cv_t<S>&, std::ranges::range_reference_t<const V>>
    { return Iterator<true>{this, std::ranges::begin(m_base)}; }

    constexpr Sentinel<false>
    end()
    { return Sentinel<false>{std::ranges::end(m_base)}; }

    constexpr Iterator<false>
    end() requires std::ranges::common_range<V>
    { return Iterator<false>{this, std::ranges::end(m_base)}; }

    constexpr Sentinel<true>
    end() const
    requires std::ranges::range<const V>
    && std::regular_invocable<const F&, std::remove_cv_t<S>&, std::ranges::range_reference_t<const V>>
    { return Sentinel<true>{std::ranges::end(m_base)}; }

    constexpr Iterator<true>
    end() const
    requires std::ranges::common_range<const V>
    && std::regular_invocable<const F&, std::remove_cv_t<S>&, std::ranges::range_reference_t<const V>>
    { return Iterator<true>{this, std::ranges::end(m_base)}; }
    
    constexpr auto
    size() requires std::ranges::sized_range<V>
    { return std::ranges::size(m_base); }

    constexpr auto
    size() const requires std::ranges::sized_range<const V>
    { return std::ranges::size(m_base); }
};

template<class R, class S, class F>
scan_view(R&&, S, F)
    -> scan_view<std::ranges::views::all_t<R>, S, F>;

namespace detail
{
    template<typename Range, typename S, typename F>
	concept can_scan_view
	    = requires { scan_view{std::declval<Range>(), std::declval<S>(), std::declval<F>()}; };

    struct scan_range_adaptor : std::ranges::views::__adaptor::_RangeAdaptor<scan_range_adaptor>
    {
        template<std::ranges::viewable_range Range, typename S, typename F>
	        requires can_scan_view<Range, S, F>
        constexpr auto
        operator() (Range&& r, S&& s, F&& f) const
        {
            return scan_view(std::forward<Range>(r), std::forward<S>(s), std::forward<F>(f));
        }

        using std::ranges::views::__adaptor::_RangeAdaptor<scan_range_adaptor>::operator();
        static constexpr int _S_arity = 3;
    };

} // namespace detail

namespace views
{
    detail::scan_range_adaptor scan;
}

} // namespace ranges

namespace views = ranges::views;

} // namespace lfr

struct Line {
    operator std::string_view() const noexcept { return m_line; }
    operator std::string() const noexcept { return m_line; }
    operator std::string&() noexcept { return m_line; }
    operator const std::string&() const noexcept { return m_line; }

private:
    std::string m_line;

    friend std::ostream& operator<<(std::ostream& os, const Line& line);
    friend std::istream& operator>>(std::istream& is, Line& line);
};

std::ostream& operator<<(std::ostream& os, const Line& line) {
    os << line.m_line;
    return os;
}

std::istream& operator>>(std::istream& is, Line& line) {
    std::istream::sentry sentry(is);
    line.m_line.clear();
    for (char c = is.get(); is; c = is.get())
    {
        if (c == '\n')
        {
            break;
        }
        line.m_line += c;
    }
    // may need to trim trailing whitespace...
    if (is.flags() & std::ios_base::skipws)
        while (!line.m_line.empty() && std::isspace(line.m_line.back()))
            line.m_line.pop_back();
    return is;
}

int main(int argc, char* argv[])
{
    if(argc < 2) {
        std::cerr << "Expected a path to input data as an argmument" << std::endl;
        return 1;
    }

    auto input_filepath = std::experimental::filesystem::path(argv[1]);

    if(!std::experimental::filesystem::exists(input_filepath)) {
        std::cerr << "File not found" << std::endl;
        return 1;
    }

    if(std::experimental::filesystem::is_directory(input_filepath)) {
        std::cerr << "Expected a file, not a directory" << std::endl;
        return 1;
    }

    // Create an input stream over the file
    auto input_stream = std::ifstream(input_filepath.c_str());
    auto input_view = std::ranges::istream_view<Line>(input_stream >> std::noskipws);

    // Parse input
    std::ranges::for_each(
        input_view
            | std::views::filter([](std::string_view v){ return true; })
            | lfr::views::scan(static_cast<size_t>(0), [](size_t& s, std::string_view v) -> std::optional<size_t> {
                if (v.empty()) {
                    auto ret = std::make_optional<size_t>(s);
                    s = 0;
                    return ret;
                } else {
                    size_t x;
                    std::string str(v);
                    std::stringstream ss(str);
                    ss >> x;
                    s += x;
                    return std::nullopt;
                }
            })
            | std::views::filter([](auto o) {
                return static_cast<bool>(o);
            })
            | std::views::transform([](auto o) { return o.value(); }),
        [](auto s){ std::cout << s << std::endl; }
    );

    return 0;
}


enum SearchFilter {
    None,
    Alphabetical,
    Popular,
    Latest,
    TopRated
}

function filter_to_string(filter: SearchFilter) {
    switch(filter) {
        case SearchFilter.Alphabetical:
            return "Alphabetical";
        
        case SearchFilter.Popular:
            return "Popular";
        
        case SearchFilter.Latest:
            return "Latest";
        
        case SearchFilter.TopRated:
            return "TopRated";
        
        case SearchFilter.None:
            return "None";
        default:
            return "Unknow";
    }
}


const genresList = [
    "Action",
    "Adventure",
    "Animation",
    "Comedy",
    "Crime",
    "Documentary",
    "Drama",
    "Family",
    "Fantasy",
    "History",
    "Horror",
    "Music",
    "Mystery",
    "Romance",
    "ScienceFiction",
    "Thriller",
    "TVMovie",
    "War",
    "Western"
]

export {
    SearchFilter,
    genresList,
    filter_to_string
}
function formatVote(vote: number) {
    return (vote * 10).toFixed(0);
}

function formatDuration(minutes: number) {
    const hours = Math.floor(minutes / 60);
    const remainingMinutes = minutes % 60;
    return `${hours}h ${remainingMinutes}m`;
}


// Ajoute un espace pour une meilleure lisibilité ex: 10000 -> 10 000
function formatWithSpace(value: number) {
    return value.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ' ');
}

function formatMoney(value: number) {
    return Intl.NumberFormat("en", {
        style: "currency",
        currency: "USD",
        notation: "compact",
    }).format(value);
}

export {
    formatVote,
    formatDuration,
    formatWithSpace,

    formatMoney,
}

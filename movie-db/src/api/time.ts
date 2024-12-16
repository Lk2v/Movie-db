function timeAgo(timestamp: number) {
    const now = Date.now(); // Temps actuel en millisecondes
    const diff = now - timestamp * 1000; // Différence en millisecondes (timestamp est souvent en secondes)

    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);
    const weeks = Math.floor(days / 7);
    const months = Math.floor(days / 30); // Approximatif
    const years = Math.floor(days / 365); // Approximatif

    if (seconds < 60) {
        return `${seconds} seconds ago`;
    } else if (minutes < 60) {
        return `${minutes} minutes ago`;
    } else if (hours < 24) {
        return `${hours} hours ago`;
    } else if (days < 7) {
        return `${days} days ago`;
    } else if (weeks < 4) {
        return `${weeks} weeks ago`;
    } else if (months < 12) {
        return `${months} months ago`;
    } else {
        return `${years} years ago`;
    }
}

function toDate(timestamp: number): Date {
    return new Date(timestamp * 1000);
}

export {
    timeAgo,
    toDate,
}
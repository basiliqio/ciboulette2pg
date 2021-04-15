INSERT INTO comments ("author", "article", "body")
    VALUES ($1, $2, 'Vous savez, moi je ne crois pas qu"il y ait de bonne ou de mauvaise situation. Moi, si je devais résumer ma vie aujourd"hui avec vous, je dirais que c"est d"abord des rencontres. Des gens qui m"ont tendu la main, peut-être à un moment où je ne pouvais pas, où j"étais seul chez moi.') RETURNING id;

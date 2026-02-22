const indicators = {}
let nextAnchorId = 0;

window.addEventListener("load", () => {
    const on_change = elem => {
        const id = elem.dataset.indicatorId;
        const indicator = document.getElementById(id);
        const indicatorRect = indicator.getBoundingClientRect();
        const rect = elem.getBoundingClientRect();

        const anchor = document.getElementById(indicator.style.getPropertyValue("position-anchor"));
        const anchorRect = anchor.getBoundingClientRect();

        indicator.style.width = `${indicatorRect.width}px`;
        indicator.style.height = `${indicatorRect.height}px`;
        indicator.offsetHeight;

        indicator.style.left = `calc(anchor(left) + ${rect.left - anchorRect.left}px)`;
        indicator.style.top = `calc(anchor(top) + ${rect.top - anchorRect.top}px)`;

        indicator.style.right = "unset";
        indicator.style.bottom = "unset";
        indicator.style.width = `${rect.width}px`;
        indicator.style.height = `${rect.height}px`;

        indicators[id].target = elem;
    }

    document.querySelectorAll("[data-indicator-id]").forEach(elem => {
        elem.style.setProperty("anchor-name", `--anchor${nextAnchorId}`);
        elem.id = elem.style.getPropertyValue("anchor-name");
        nextAnchorId++;


        elem.addEventListener("change", ev => {
            console.log(ev);
            on_change(elem);
        });
    });

    document.querySelectorAll("[data-indicator]").forEach(indicator => {
        const listener = () => {
            const target = indicators[indicator.id].target;

            indicator.style.transition = "none";
            indicator.offsetHeight;

            if (target)  {
                const anchorName = target.style.getPropertyValue("anchor-name");
                console.log(anchorName);
                indicator.style.setProperty("position-anchor", anchorName);

                indicator.style.left = "calc(anchor(left) + 0px)";
                indicator.style.top = "calc(anchor(top) + 0px)";
                indicator.style.right = "anchor(right)";
                indicator.style.bottom = "anchor(bottom)";
                indicator.style.width = "unset";
                indicator.style.height = "unset";

                indicator.offsetHeight;
            }

            indicator.style.transition = "";
        };

        indicator.addEventListener("transitionend", listener);

        const target = document.querySelector("[data-indicator-id]:has(:checked)");

        console.log("default option: ", target);

        indicators[indicator.id] = {
            elem: indicator,
            target,
        };

        listener();
        on_change(target);
    });
});

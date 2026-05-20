(() => {
  const data = window.TUI_ADV_IMPLEMENTATION;
  const $ = (selector, root = document) => root.querySelector(selector);
  const $$ = (selector, root = document) => Array.from(root.querySelectorAll(selector));

  let activeFeatureFilter = "all";
  let activeNodeId = data.routes.nodes[0]?.id;
  let query = "";

  const normalize = (value) => String(value ?? "").toLowerCase();
  const includesQuery = (parts) => !query || parts.some((part) => normalize(part).includes(query));

  function init() {
    renderShell();
    renderOverview();
    renderStatus();
    renderFeatureFilters();
    renderFeatures();
    renderRoutes();
    renderContent();
    renderSources();
    bindSearch();
    observeSections();
  }

  function renderShell() {
    const nav = $("#section-nav");
    const sections = $$("[data-section-title]");
    nav.innerHTML = sections.map((section, index) => `
      <a href="#${section.id}" ${index === 0 ? 'aria-current="true"' : ""}>${section.dataset.sectionTitle}</a>
    `).join("");
  }

  function renderOverview() {
    $("#project-title").textContent = data.overview.title;
    $("#project-summary").textContent = data.overview.summary;
    $("#hero-meta").innerHTML = data.overview.metrics.map((metric) => `
      <div class="metric-card">
        <strong>${metric.value}</strong>
        <span>${metric.label}</span>
      </div>
    `).join("");
  }

  function renderStatus() {
    const groups = [
      { title: "완료된 핵심", items: data.overview.highlights },
      { title: "다음 후보", items: data.overview.nextWork },
      { title: "업데이트 방식", items: data.overview.maintenance }
    ];
    $("#status-grid").innerHTML = groups.map((group) => `
      <article class="status-card">
        <h3>${group.title}</h3>
        <ul>${group.items.map((item) => `<li>${item}</li>`).join("")}</ul>
      </article>
    `).join("");
  }

  function renderFeatureFilters() {
    const categories = ["all", ...new Set(data.systems.features.map((feature) => feature.category))];
    $("#feature-filters").innerHTML = categories.map((category) => `
      <button type="button" data-filter="${category}" aria-pressed="${category === activeFeatureFilter}">${categoryLabel(category)}</button>
    `).join("");
    $$("#feature-filters button").forEach((button) => {
      button.addEventListener("click", () => {
        activeFeatureFilter = button.dataset.filter;
        renderFeatureFilters();
        renderFeatures();
      });
    });
  }

  function renderFeatures() {
    const features = data.systems.features.filter((feature) => {
      const filterMatch = activeFeatureFilter === "all" || feature.category === activeFeatureFilter;
      const queryMatch = includesQuery([feature.title, feature.description, feature.category, feature.files.join(" "), feature.tags.join(" ")]);
      return filterMatch && queryMatch;
    });

    $("#feature-grid").innerHTML = features.map((feature) => `
      <article class="card">
        <h3>${highlight(feature.title)}</h3>
        <p>${highlight(feature.description)}</p>
        <div class="tag-row">${feature.tags.map((tag) => `<span class="tag">${highlight(tag)}</span>`).join("")}</div>
        <footer>${feature.files.map((file) => `<code>${highlight(file)}</code>`).join(" ")}</footer>
      </article>
    `).join("") || emptyState("검색 결과가 없습니다.");
  }

  function renderRoutes() {
    const graph = $("#route-graph");
    const nodes = data.routes.nodes.filter((node) => includesQuery([node.label, node.kind, node.summary, node.files.join(" ")]));
    graph.innerHTML = nodes.map((node) => `
      <button class="node-button" type="button" data-node-id="${node.id}" data-kind="${node.kind}" aria-pressed="${node.id === activeNodeId}" role="listitem">
        <strong>${highlight(node.label)}</strong>
        <span>${highlight(node.summary)}</span>
      </button>
    `).join("") || emptyState("검색에 맞는 노드가 없습니다.");

    $$(".node-button", graph).forEach((button) => {
      button.addEventListener("click", () => {
        activeNodeId = button.dataset.nodeId;
        renderRoutes();
      });
    });

    renderNodeDetail();
    renderRouteList();
  }

  function renderNodeDetail() {
    const node = data.routes.nodes.find((item) => item.id === activeNodeId) || data.routes.nodes[0];
    if (!node) return;
    $("#node-detail").innerHTML = `
      <span class="eyebrow">selected node</span>
      <h3>${node.label}</h3>
      <p>${node.summary}</p>
      <dl>
        <div><dt>종류</dt><dd>${categoryLabel(node.kind)}</dd></div>
        <div><dt>관련 파일</dt><dd>${node.files.map((file) => `<code>${file}</code>`).join(" ")}</dd></div>
        <div><dt>연결 노드</dt><dd>${node.links.map((id) => labelForNode(id)).join(", ") || "없음"}</dd></div>
      </dl>
    `;
  }

  function renderRouteList() {
    const routes = data.routes.routes.filter((route) => includesQuery([route.title, route.type, route.outcome, route.steps.join(" "), route.command || ""]));
    $("#route-list").innerHTML = routes.map((route) => `
      <article class="route-item">
        <div class="tag-row"><span class="tag">${categoryLabel(route.type)}</span><span class="tag">${route.status}</span></div>
        <h3>${highlight(route.title)}</h3>
        <p>${highlight(route.outcome)}</p>
        <ol>${route.steps.map((step) => `<li>${highlight(step)}</li>`).join("")}</ol>
        ${route.command ? `<p><code>${highlight(route.command)}</code></p>` : ""}
      </article>
    `).join("") || emptyState("검색 결과가 없습니다.");
  }

  function renderContent() {
    $("#content-columns").innerHTML = data.content.catalog.map((group) => `
      <article class="content-column">
        <h3>${group.title} <span class="tag">${group.items.length}</span></h3>
        <ul class="content-list">
          ${group.items.map((item) => `<li><strong>${highlight(item.name)}</strong><span>${highlight(item.id)}</span></li>`).join("")}
        </ul>
      </article>
    `).join("");
  }

  function renderSources() {
    $("#source-grid").innerHTML = data.overview.sources.map((source) => `
      <article class="source-card">
        <h3>${source.title}</h3>
        <p>${source.description}</p>
        <footer><code>${source.path}</code></footer>
      </article>
    `).join("");
  }

  function bindSearch() {
    $("#global-search").addEventListener("input", (event) => {
      query = normalize(event.target.value.trim());
      renderFeatures();
      renderRoutes();
    });
  }

  function observeSections() {
    const navLinks = $$("#section-nav a");
    const byId = Object.fromEntries(navLinks.map((link) => [link.getAttribute("href").slice(1), link]));
    const observer = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (!entry.isIntersecting) return;
        navLinks.forEach((link) => link.removeAttribute("aria-current"));
        byId[entry.target.id]?.setAttribute("aria-current", "true");
      });
    }, { rootMargin: "-30% 0px -55% 0px" });
    $$("[data-section-title]").forEach((section) => observer.observe(section));
  }

  function labelForNode(id) {
    return data.routes.nodes.find((node) => node.id === id)?.label || id;
  }

  function categoryLabel(value) {
    return ({
      all: "전체",
      engine: "엔진",
      tui: "TUI",
      save: "저장",
      content: "콘텐츠",
      safety: "안전",
      test: "테스트",
      escape: "탈출",
      conquest: "정복",
      truth: "진실",
      reality: "현실 연결"
    })[value] || value;
  }

  function emptyState(message) {
    return `<article class="card"><h3>${message}</h3><p>검색어를 줄이거나 다른 카테고리를 선택해 주세요.</p></article>`;
  }

  function highlight(text) {
    const raw = String(text ?? "");
    if (!query) return escapeHtml(raw);
    const lower = raw.toLowerCase();
    const index = lower.indexOf(query);
    if (index < 0) return escapeHtml(raw);
    return `${escapeHtml(raw.slice(0, index))}<mark>${escapeHtml(raw.slice(index, index + query.length))}</mark>${escapeHtml(raw.slice(index + query.length))}`;
  }

  function escapeHtml(text) {
    return text.replace(/[&<>"]/g, (char) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[char]));
  }

  init();
})();

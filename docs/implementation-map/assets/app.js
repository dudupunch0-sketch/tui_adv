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
    const nodes = data.routes.nodes;
    graph.innerHTML = `
      <div class="graph-map" role="list" aria-label="연결선이 있는 루트 노드 지도">
        ${renderGraphLines(nodes)}
        ${nodes.map(renderNodeButton).join("")}
      </div>
    `;

    $$(".node-button", graph).forEach((button) => {
      button.addEventListener("click", () => {
        activeNodeId = button.dataset.nodeId;
        renderRoutes();
      });
    });

    renderNodeDetail();
    renderRouteList();
  }

  function renderGraphLines(nodes) {
    const nodeById = new Map(nodes.map((node) => [node.id, node]));
    const edges = [];
    const seen = new Set();
    nodes.forEach((source) => {
      const targetIds = new Set([
        ...(source.links || []),
        ...(source.scene?.choices || [])
          .map((choice) => choice.next)
          .filter((id) => id && id !== source.id && nodeById.has(id))
      ]);
      targetIds.forEach((targetId) => {
        if (targetId === source.id) return;
        const target = nodeById.get(targetId);
        if (!target) return;
        const key = [source.id, target.id].sort().join("::");
        if (seen.has(key)) return;
        seen.add(key);
        edges.push({ source, target });
      });
    });

    return `
      <svg class="graph-lines" viewBox="0 0 100 100" preserveAspectRatio="none" aria-hidden="true">
        ${edges.map(({ source, target }) => {
          const active = source.id === activeNodeId || target.id === activeNodeId;
          const dimmed = query && !nodeMatchesQuery(source) && !nodeMatchesQuery(target);
          return `<path class="graph-line ${active ? "is-active" : ""} ${dimmed ? "is-dimmed" : ""}" data-kind="${source.kind}" d="${edgePath(source, target)}" />`;
        }).join("")}
      </svg>
    `;
  }

  function renderNodeButton(node) {
    const active = node.id === activeNodeId;
    const related = isRelatedToActive(node);
    const filtered = query && !nodeMatchesQuery(node);
    return `
      <button class="node-button flow-node ${related ? "is-related" : ""} ${filtered ? "is-filtered-out" : ""}" type="button"
        data-node-id="${node.id}" data-kind="${node.kind}" aria-pressed="${active}"
        style="--x: ${node.x}%; --y: ${node.y}%;" role="listitem"
        aria-label="${escapeHtml(node.label)} 노드 보기">
        <strong>${highlight(node.label)}</strong>
        <span>${highlight(node.summary)}</span>
      </button>
    `;
  }

  function edgePath(source, target) {
    const curve = Math.max(6, Math.abs(target.x - source.x) * 0.32);
    const c1x = source.x + curve;
    const c2x = target.x - curve;
    return `M ${source.x} ${source.y} C ${c1x} ${source.y}, ${c2x} ${target.y}, ${target.x} ${target.y}`;
  }

  function renderNodeDetail() {
    const node = data.routes.nodes.find((item) => item.id === activeNodeId) || data.routes.nodes[0];
    if (!node) return;
    const choices = node.scene?.choices || [];
    const graphNextIds = collectGraphNextIds(node);
    const endpointNext = collectEndpointNext(node);

    $("#node-detail").innerHTML = `
      <span class="eyebrow">selected node</span>
      <h3>${highlight(node.label)}</h3>
      <p>${highlight(node.summary)}</p>

      <section class="scene-block">
        <h4>배경</h4>
        <p>${highlight(node.scene?.background || "아직 배경 설명이 등록되지 않았습니다.")}</p>
      </section>

      <section class="scene-block dialogue-block">
        <h4>대사</h4>
        <p>${highlight(node.scene?.dialogue || "아직 대사나 상황문이 등록되지 않았습니다.")}</p>
      </section>

      <section class="scene-block">
        <h4>선택지</h4>
        <div class="choice-list">
          ${choices.map(renderChoiceCard).join("") || `<p>이 노드는 엔딩 또는 설명용 노드라 선택지가 없습니다.</p>`}
        </div>
      </section>

      <section class="scene-block">
        <h4>가능한 다음 상황</h4>
        <div class="next-node-list">
          ${graphNextIds.map((id) => renderNextNodeButton(id)).join("")}
          ${endpointNext.map((item) => `<span class="next-chip next-chip-static">${highlight(item.label)}</span>`).join("")}
          ${graphNextIds.length + endpointNext.length === 0 ? `<span class="next-chip next-chip-static">루트 종료</span>` : ""}
        </div>
      </section>

      <dl>
        <div><dt>종류</dt><dd>${categoryLabel(node.kind)}</dd></div>
        <div><dt>관련 파일</dt><dd>${node.files.map((file) => `<code>${highlight(file)}</code>`).join(" ")}</dd></div>
      </dl>
    `;

    $$(".next-node-button", $("#node-detail")).forEach((button) => {
      button.addEventListener("click", () => {
        activeNodeId = button.dataset.nodeId;
        renderRoutes();
      });
    });
  }

  function renderChoiceCard(choice, index) {
    const nextNode = data.routes.nodes.find((node) => node.id === choice.next);
    const nextLabel = nextNode ? nextNode.label : (choice.nextLabel || choice.next);
    return `
      <article class="choice-card">
        <div class="choice-index">${String(index + 1).padStart(2, "0")}</div>
        <div>
          <h5>${highlight(choice.label)}</h5>
          <p>${highlight(choice.line)}</p>
          <dl>
            <div><dt>조건</dt><dd>${highlight(choice.requirements || "없음")}</dd></div>
            <div><dt>결과</dt><dd>${highlight(choice.result || "상태 변화 없음")}</dd></div>
            <div><dt>다음</dt><dd>${nextNode ? `<button class="inline-node-link next-node-button" type="button" data-node-id="${choice.next}">${highlight(nextLabel)}</button>` : `<span>${highlight(nextLabel || "루트 종료")}</span>`}</dd></div>
          </dl>
        </div>
      </article>
    `;
  }

  function renderNextNodeButton(id) {
    const node = data.routes.nodes.find((item) => item.id === id);
    if (!node) return "";
    return `<button class="next-node-button next-chip" type="button" data-node-id="${id}">${highlight(node.label)}</button>`;
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

  function collectGraphNextIds(node) {
    const graphIds = new Set(data.routes.nodes.map((item) => item.id));
    return [...new Set((node.scene?.choices || [])
      .map((choice) => choice.next)
      .filter((id) => id && id !== node.id && graphIds.has(id)))];
  }

  function collectEndpointNext(node) {
    const graphIds = new Set(data.routes.nodes.map((item) => item.id));
    const endpoints = new Map();
    (node.scene?.choices || []).forEach((choice) => {
      if (!choice.next) return;
      if (graphIds.has(choice.next)) {
        if (choice.next === node.id) endpoints.set("current", "현재 상황 유지");
        return;
      }
      endpoints.set(choice.next, choice.nextLabel || choice.next);
    });
    return [...endpoints.entries()].map(([id, label]) => ({ id, label }));
  }

  function isRelatedToActive(node) {
    if (node.id === activeNodeId) return false;
    const active = data.routes.nodes.find((item) => item.id === activeNodeId);
    if (!active) return false;
    const activeLinks = active.links || [];
    const nodeLinks = node.links || [];
    const activeToNode = activeLinks.includes(node.id) || (active.scene?.choices || []).some((choice) => choice.next === node.id);
    const nodeToActive = nodeLinks.includes(active.id) || (node.scene?.choices || []).some((choice) => choice.next === active.id);
    return activeToNode || nodeToActive;
  }

  function nodeMatchesQuery(node) {
    const choices = node.scene?.choices || [];
    return includesQuery([
      node.label,
      node.kind,
      node.summary,
      node.files.join(" "),
      node.scene?.background || "",
      node.scene?.dialogue || "",
      choices.map((choice) => [choice.label, choice.line, choice.requirements, choice.result, choice.next].join(" ")).join(" ")
    ]);
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
    return String(text).replace(/[&<>"]/g, (char) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[char]));
  }

  init();
})();

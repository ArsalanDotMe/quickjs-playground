async function onRequestGet({ params }) {
  console.log("onRequestGet", params);
  const res = await fetch(`https://rickandmortyapi.com/api/character/${params.id}`);
  console.log("res", res);
  const data = await res.json();
  console.log("data", data);
  const info = JSON.stringify(data, null, 2);
  console.log("info", info);
  return new Response(info);
}

module.exports = {
  onRequestGet,
};
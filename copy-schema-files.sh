#!/bin/bash

shopt -s globstar

submodule_path="podping-schemas"
package_path="src"

mkdir -p "${package_path}"

readarray -d '' paths_to_copy < <(find "${submodule_path}/schema/" -mindepth 1 -maxdepth 1 -print0)

for path in "${paths_to_copy[@]}"
do
    cp -r "${path}" "${package_path}"
done

find "src" -name '*.capnp' -type f -exec sed -i -E 's/\/schema/\/src/g' {} \;

for file in "${package_path}"/**/*.capnp; do
    if [[ "${file}" == */rust.capnp ]]
    then
        # No need to add rust.capnp to public modules
        continue
    fi

    file_dir=$(dirname "$file")

    mapfile -t file_imports < <(grep -oP 'using import "\K[^"].*?(?=\/[^\/]*\.capnp)' "$file" | uniq)

    for import_dir in "${file_imports[@]}"
    do
        relpath=$(realpath --relative-to="$file_dir" "./$import_dir")

        escaped_import_dir=${import_dir//\//\\\/}
        escaped_relpath_dir=${relpath//\//\\\/}
        escaped_relpath_dir=${escaped_relpath_dir//\\./\\\./}

        sed -i -e "s/${escaped_import_dir}/${escaped_relpath_dir}/g" "${file}"
    done

    mod_dir=$(dirname "$file")
    filename=$(basename -- "$file")

    echo "pub mod ${filename%%.*}_capnp;" >> "$mod_dir/mod.rs"
done


readarray -d '' lib_module_dirs < <(find "${package_path}" -mindepth 1 -maxdepth 1 -type d -print0)

for lib_module_dir in "${lib_module_dirs[@]}"
do
    echo "pub mod $(basename "${lib_module_dir}");" >> "${package_path}/lib.rs"
done

readarray -d '' mod_module_dirs < <(find "${package_path}" -mindepth 2 -type d -print0)

for mod_module_dir in "${mod_module_dirs[@]}"
do
    echo "pub mod $(basename "${mod_module_dir}");" >> "$(dirname "${mod_module_dir}")/mod.rs"
done

for file in "${package_path}"/**/*.json; do
    dir=$(dirname "$file")
    filename=$(basename -- "$file")
    new_filename="${dir}/${filename%%.*}_json.json"

    mv "${file}" "$new_filename"

    cargo typify "$new_filename"

    echo "pub mod ${filename%%.*}_json;" >> "${dir}/mod.rs"
done
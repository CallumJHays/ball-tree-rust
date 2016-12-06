cheap_best_sib (nl:BLT_ND):BLT_ND is
    -- A cheap guess at the best sibling node for inserting new leaf nl.
    local bcost:REAL;-- best cost = node vol + ancestor expansion
        ae:REAL;-- accumulated ancestor expansion
        nd:BLT_ND; done:BOOLEAN; lv,rv,wv:REAL;
    do
        if tree.Void then-- Result.Void means tree is void
        else
            Result:=tree; tb.to_bound_balls(tree.bl,nl.bl); wv:=tb.pvol;
            bcost := wv;
            ae:=0.;-- ancestor expansion starts at zero.
            from nd:=tree until nd.leaf or done loop
            ae:=ae+wv-nd.pvol; -- correct for both children
            if ae>=bcost then done:=true -- can’t do any better
            else
                tb.to_bound_balls(nd.lt.bl,nl.bl); lv:=tb.pvol;
                tb.to_bound_balls(nd.rt.bl,nl.bl); rv:=tb.pvol;
                if ae+lv<=bcost then Result:=nd.lt; bcost:=ae+lv; end;
                if ae+rv<=bcost then Result:=nd.rt; bcost:=ae+rv; end;
                if lv-nd.lt.pvol<=rv-nd.rt.pvol -- left expands less
                then wv:=lv; nd:=nd.lt;
                else wv:=rv; nd:=nd.rt; end;
            end; -- if
        end; -- loop
    end; -- if
end; -- cheap_best_sib
